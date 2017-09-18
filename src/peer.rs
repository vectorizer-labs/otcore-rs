use std::collections::BTreeSet;
use ot_set::OTSet;
use doc_state::{DocState, Operation};

struct Peer
{
    revision_id : usize, // a linearly incremented revision id
    context : BTreeSet<usize> //the context of revisions this peer exists in
}

impl Peer
{
    fn new() -> Peer
    {
        return Peer
        {
            revision_id : 0,
            context : BTreeSet::new()
        }
    }
    
    fn merge_op(&mut self, doc_state : DocState, op : Operation)
    {
        
        //we already have this, roll rev forward
        if self.revision < doc_state.operations.len() && doc_state.operations[self.revision_id].id == op.id 
        {
            self.revision_id += 1;
            //
            while self.revision_id < doc_state.operations.len() 
                && self.context.contains(doc_state.operations[self.revision_id].id)
            {
                self.context.remove(doc_state.operations[self.revision_id].id);
                self.revision_id += 1;
            }
            return;
        }
            
        // we already have this, but can't roll rev forward
        for i in self.revision_id..doc_state.operations.len()
        {
            if doc_state.operations[i].id == op.id
            {
                self.context.insert(op.id.clone());
                return;
            }
        }
            
        
        // we don't have it, need to merge
        //for all the operations that are potentially in conflict 
        //merge them
        let insert_list : Vec<(usize,usize)> = Vec::new();
        let S : BTreeSet<usize> = BTreeSet::new();
        let T : BTreeSet<usize> = BTreeSet::new();
        for ix in (doc_state.operations.len()-1..self.revision_id).rev()
        {
            let doc_space_index : usize = S.get_doc_space_index(doc_state.operations[ix].id);
            //if the operation is an insert
            if doc_state.operations[ix].is_insert 
            {
                //if our context does not contain this id
                if !self.context.contains(doc_state.operations[self.revision_id].id)
                {
                    insert_list.push(
                        T.get_user_space_index(doc_state.operations[ix].id),
                        doc_state.operations[ix].user_id
                    );
                    T.remove(doc_space_index);
                }
                S.remove(doc_space_index);
            }
        }
        
        for i in (insert_list.len()-1..0).rev()
        {
            op = transform_ins(op, insert_list[i][0], insert_list[i][1]);
        }
        
        
        if this.rev == ops.length
        {
            self.revision_id +=1;
        }else
        {
            self.context.push(op.id)
        }
        doc_state.operations.push(op);
    }
    
    fn transform_insertion(op1 : Operation, index : usize, user : usize)
    {
        if(op1.is_insert)
        {
            if op1.index < index || (op1.index == index && op1.user = user)
            {
                return op1;
            }
        }else
        {
            if op1.index < index
            {
                return op1;
            }
            return Operation::new();
        }
    }
    
}

function transform_ins(op1, ix, pri) {
	if (op1.ty == 'ins') {
		if (op1.ix < ix || (op1.ix == ix && op1.pri < pri)) {
			return op1;
		}
		return {ty: op1.ty, ix: op1.ix + 1, ch: op1.ch, pri: op1.pri, id: op1.id};
	} else { // op1.ty is del
		if (op1.ix < ix) {
			return op1;
		}
		return {ty: op1.ty, ix: op1.ix + 1, id: op1.id};
	}
}

//class Peer {
//	constructor(string)
//    {
//		this.rev = 0;
//        this.context = new Set();
//	}
//
//	//merge_op(doc_state, op)
//    //{
//		var id = op.id;
//		var ops = doc_state.ops;
//        
//        // we already have this, roll rev forward
//		if (this.rev < ops.length && ops[this.rev].id == id) {
//			this.rev++;
//			while (this.rev < ops.length && this.context.has(ops[this.rev].id)) {
//				this.context.delete(ops[this.rev].id);
//				this.rev++;
//			}
//			return;
//		}
//  /      
//        // we already have this, but can't roll rev forward
//		for (var ix = this.rev; ix < ops.length; ix++) {
//			if (ops[ix].id == id) {
//				this.context.add(id);
//				return;
//			}
//		}
//        
//		// we don't have it, need to merge
//		var ins_list = [];
//		var S = null;
//		var T = null;
//		for (var ix = ops.length - 1; ix >= this.rev; ix--) {
//			var my_op = ops[ix];
//			if (my_op.ty == 'ins') {
//				var i = xi(S, my_op.ix);
//				if (!this.context.has(my_op.id)) {
//					ins_list.push([xi_inv(T, i), my_op.pri]);
//					T = union_one(T, i);
//				}
//				S = union_one(S, i);
//			}
//		}
        
        //
//		for (var i = ins_list.length - 1; i >= 0; i--) {
//			op = transform_ins(op, ins_list[i][0], ins_list[i][1]);
//		}
        
        //
//        var current = (this.rev == ops.length);
//		doc_state.add(op);
//        if (current) {
//            this.rev++;
//        } else {
//            this.context.add(id);
//        }
//	}
//}