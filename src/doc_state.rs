use std::collections::BTreeSet;       
use ot_set::OTSet;
use operation::Operation;

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

pub struct DocState<T : Clone> {
    pub log : Vec<Operation<T>>,
    pub deletes : BTreeSet<usize>,
    pub string : String,
    pub points : Vec<usize>
}



impl DocState<char>
{

  pub fn new() -> DocState<char>
  {
        let doc : DocState<char> = DocState 
        {
            log : Vec::new(),
            deletes : BTreeSet::new(),
            string : String::new(),
            points : Vec::new()
        };
        doc
  }

  pub fn add(&mut self, O: Operation<char>)
  {
    self.log.push(O.clone());
    //if its an insert call insertIntoString
    //else deleteFromString
    match &O.is_insert
    {
      true => self.insertIntoString(O),
      false => self.deleteFromString(O)
    };
  }

  fn deleteFromString(&mut self, O: Operation<char>)
  {
    //if we already deleted the character in question then 
    //ignore this operation because the effect is the same 
    if self.deletes.contains(&O.index) { return; }

    //otherwise get the log space index
	  let index : usize = self.deletes.getLogSpaceIndex(O.index);

    //delete it
	  self.deletes.insert(O.index);

    //modify the actual string to reflect the change
	  self.string.remove(index);

    //update the points in the UI view
    //move back every character after the delete by one
    for i in 0..self.points.len() { if self.points[i] > index { self.points[i] -= 1;} }
  }

  fn insertIntoString(&mut self, O: Operation<char>)
  {
    //update the deletes 
    //by incrementing all the delete indexes after our insert by one
    self.deletes.incrementIndicesPastIndex(&O.index);

    //get the string space index of our insert now that we've updated
    let index : usize = self.deletes.getStringSpaceIndex(O.index);

    //modify the actual string to reflect the change
    self.string.insert(index, O.object);

    //update the points in the UI view
    //move every character after the insert up by one
    for i in 0..self.points.len() { if self.points[i] > index { self.points[i] += 1;} }
  }

  //gets the log space index for this docstate
  //used for creating new insert operations in the GUI
 pub fn getLogSpaceIndex(&mut self, index : usize) -> usize { self.deletes.getLogSpaceIndex(index) }

}
    