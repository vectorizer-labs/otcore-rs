use std::collections::BTreeSet;
use operation::Operation;
use list::List;
use ot_set::OTSet;

// Copyright 2016 Google Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//This algorithm from https://github.com/google/ot-crdt-papers/blob/master/ot_toy.js
// has been adapted to a more readable version below that includes 
//references from the source material by Raph Levien and the GOTO paper:
//http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.53.933&rep=rep1&type=pdf

pub struct Peer<>
{
    revision : usize, // a linearly incremented revision id
    context : BTreeSet<usize>, //the context of revisions this peer exists in
}

impl Peer
{
  pub fn new() -> Peer
  {
      return Peer
      {
          revision : 0,
          context : BTreeSet::new()

      }
  }

  //Handles 3 cases as seen in the GOTO paper
  pub fn mergeOP<T : Clone>(&mut self, O : Operation<T>, dc : &List<T>) -> Operation<T>
  {
    if self.repeatOP(&O, dc)  {return O;}
    else if self.case2(&O, dc) { return O;}
    else { return self.merge(O, dc); }
  }


  //we already have this operation in the log of our List
  //but a peer sent us a duplicate
  fn repeatOP<T : Clone>(&mut self, O : &Operation<T>, dc : &List<T>) -> bool
  {
    //the log is
    if self.revision < dc.log.len() && dc.log[self.revision].equals(&O)
    {
			// we already have this, roll peer revision forward
			self.revision += 1;
			while self.revision < dc.log.len() && self.context.contains(&dc.log[self.revision].id)
      {
				self.context.remove(&dc.log[self.revision].id);
				self.revision+=1;
			}
			return true;
		}

    return false;
  }

  //
  fn case2<T : Clone>(&mut self, O : &Operation<T>, dc : &List<T>) -> bool
  {
    
    for ix in self.revision..dc.log.len()
    {
      // we already have this Operation, but can't roll revision forward
      if dc.log[ix].equals(&O)
      {
        self.context.insert(O.id);
        return true;
      }
    }
    return false;
  }


  fn merge<T : Clone>(&mut self, mut O : Operation<T>, dc : &List<T>) -> Operation<T>
  {
    // we don't have it, need to merge
		let mut ins_list: Vec<(usize, usize)> = Vec::new();

    //TODO: make T and S struct members and clear them each time we do a merge to avoid overhead
    //a list of all operations from the end of the log to this.revision
    //with their state as if all operations were undone back to this.revision  
    let mut T : BTreeSet<usize> = BTreeSet::new();

    //a list of all operations CONCURRENT TO O
    // from the end of the log to this.revision
    //these operations are transformed against the non concurrent members of set T
    //until all the concurrent operations are at the end of the log
		let mut S : BTreeSet<usize> = BTreeSet::new();

    //loop backwards through the log building up patches S and T
    //until we reach the revision this context is up to date on
		for ix in (self.revision..dc.log.len()).rev()
    {
			let current : &Operation<T> = &dc.log[ix];

      //we don't have to care about delete operations because tombstones
			if !current.is_insert { continue; }

      //handle the insert
      //get the real log space index
			let i : usize = S.getLogSpaceIndex(current.index);

			if !self.context.contains(&current.id)
      {
				ins_list.push((T.getStringSpaceIndex(i), current.user_id));
				T.insert(i);
			}
			S.insert(i);
		}

    //transform O through the log
		for i in (0..ins_list.len()).rev() { O.transform_ins(ins_list[i].0, ins_list[i].1); }

    return O;
  }
  
  


}

