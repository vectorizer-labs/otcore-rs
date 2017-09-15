use std::collections::BTreeSet;
use ot_set::OTSet;

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
    
    fn merge_op(&mut self, doc_state : doc_state::DocState, op : doc_state::Operation)
    {
        
        //we already have this, roll rev forward
        if(self.revision < doc_state.operations.len() && docstate.operations[self.revision_id].id == op.id)
        {
            self.revision_id += 1;
            //
            while(self.revision_id < doc_state.operations.len() 
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
            if docstate.operations[i].id == op.id
            {
                self.context.insert(op.id.clone());
                return;
            }
        }
            
        // we don't have it, need to merge
        for ix in (doc_state.operations.len()-1..self.revision_id).rev()
        {
            if(doc_state.operations[ix].is_insert)
            {
                
            }
        }
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
				var i = xi(S, my_op.ix);
				if (!this.context.has(my_op.id)) {
					ins_list.push([xi_inv(T, i), my_op.pri]);
					T = union_one(T, i);
				}
				S = union_one(S, i);
			}
		}
        
        //
		for (var i = ins_list.length - 1; i >= 0; i--) {
			op = transform_ins(op, ins_list[i][0], ins_list[i][1]);
		}
        
        //
        var current = (this.rev == ops.length);
		doc_state.add(op);
        if (current) {
            this.rev++;
        } else {
            this.context.add(id);
        }
	}
}