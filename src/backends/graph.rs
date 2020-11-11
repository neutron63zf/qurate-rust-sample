#![allow(dead_code)]
#![allow(unused_variables)]
use crate::core::*;
use std::any::Any;
use std::rc::Rc;

// 計算グラフ
pub struct Graph {}
impl QBackend for Graph {}

// 中身にはアクセスできないのでセーフ
#[derive(Debug)]
pub struct GraphGateOutput {
    index: u32,
    gate: Rc<dyn Any>,
}

struct GraphPublicGraph {}
impl QPublicGraph<Graph> for GraphPublicGraph {
    // Graphのmeasureを直接実行しても計算は行われないのでこれで良い
    fn measure(&self) -> QComputationalBasis {
        QComputationalBasis::Zero
    }
}

#[derive(Debug)]
pub enum GraphQubit {
    Computational(QComputationalBasis),
    GateOutput(GraphGateOutput),
}
impl Qubit<Graph> for GraphQubit {
    fn inspect(&self) -> Box<dyn QPublicGraph<Graph>> {
        // TODO
        Box::new(GraphPublicGraph {})
    }
    fn associate(&self, qs: Vec<Box<&dyn Qubit<Graph>>>) -> Vec<Box<dyn QPublicGraph<Graph>>> {
        // TODO
        vec![Box::new(GraphPublicGraph {})]
    }
}

#[derive(Debug)]
pub struct GraphHadamard {
    input: Rc<<GraphHadamard as QGate<Graph>>::Input>,
}
impl QGate<Graph> for GraphHadamard {
    type Input = Box<GraphQubit>;
    type Output = Box<GraphQubit>;
    fn apply(q: Self::Input) -> Self::Output {
        let gate = Rc::new(GraphHadamard { input: Rc::new(q) });
        let gate_output = GraphGateOutput { index: 0, gate };
        Box::new(GraphQubit::GateOutput(gate_output))
    }
}

#[derive(Debug)]
pub struct GraphCNOT {
    input: Rc<<GraphCNOT as QGate<Graph>>::Input>,
}
impl QGate<Graph> for GraphCNOT {
    type Input = (Box<GraphQubit>, Box<GraphQubit>);
    type Output = Self::Input;
    fn apply(q: Self::Input) -> Self::Output {
        let input = Rc::new(q);
        let gate = Rc::new(GraphCNOT { input });
        let gate_output1 = GraphGateOutput {
            index: 0,
            gate: gate.clone(),
        };
        let gate_output2 = GraphGateOutput {
            index: 1,
            gate: gate.clone(),
        };
        (
            Box::new(GraphQubit::GateOutput(gate_output1)),
            Box::new(GraphQubit::GateOutput(gate_output2)),
        )
    }
}

#[derive(Debug)]
pub struct GraphCZ {
    input: Rc<<GraphCZ as QGate<Graph>>::Input>,
}
impl QGate<Graph> for GraphCZ {
    type Input = (Box<GraphQubit>, Box<GraphQubit>);
    type Output = Self::Input;
    fn apply(q: Self::Input) -> Self::Output {
        let input = Rc::new(q);
        let gate = Rc::new(GraphCZ { input });
        let gate_output1 = GraphGateOutput {
            index: 0,
            gate: gate.clone(),
        };
        let gate_output2 = GraphGateOutput {
            index: 1,
            gate: gate.clone(),
        };
        (
            Box::new(GraphQubit::GateOutput(gate_output1)),
            Box::new(GraphQubit::GateOutput(gate_output2)),
        )
    }
}
