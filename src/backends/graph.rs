#![allow(dead_code)]
#![allow(unused_variables)]
use crate::core::*;
use std::rc::Rc;

// バックエンド宣言
// 計算グラフ
pub struct Graph {}
impl QBackend for Graph {}

// 中身にはアクセスできないのでセーフ
#[derive(Debug)]
pub struct GraphGateOutput {
    index: u32,
    gate: Rc<dyn QInspectable<Graph>>,
}

// 計算グラフの公開された表現
pub enum GraphPublicGraph {
    Gate {
        gate_name: String,
        input: Vec<GraphPublicGraph>,
    },
    Basis(QBasis),
}
impl QPublicGraph<Graph> for GraphPublicGraph {
    // Graphのmeasureを直接実行しても計算は行われないのでこれで良い
    fn measure(&self) -> QBasis {
        QBasis::Zero
    }
}

// Qubit
#[derive(Debug)]
pub enum GraphQubit {
    Basis(QBasis),
    Gate(GraphGateOutput),
}
impl Qubit<Graph> for GraphQubit {}

pub fn init_graph_qubit(b: QBasis) -> Box<GraphQubit> {
    Box::new(GraphQubit::Basis(b))
}

// H ゲート
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
        Box::new(GraphQubit::Gate(gate_output))
    }
}

// CNOT ゲート
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
            Box::new(GraphQubit::Gate(gate_output1)),
            Box::new(GraphQubit::Gate(gate_output2)),
        )
    }
}

// CZ ゲート
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
            Box::new(GraphQubit::Gate(gate_output1)),
            Box::new(GraphQubit::Gate(gate_output2)),
        )
    }
}

// 計算グラフ化を行う処理
impl QInspectable<Graph> for GraphQubit {
    fn inspect(&self) -> Box<dyn QPublicGraph<Graph>> {
        // TODO
        Box::new(GraphPublicGraph::Basis(QBasis::Zero))
    }
}
impl QInspectable<Graph> for GraphHadamard {
    fn inspect(&self) -> Box<dyn QPublicGraph<Graph>> {
        // TODO
        Box::new(GraphPublicGraph::Basis(QBasis::Zero))
    }
}
impl QInspectable<Graph> for GraphCNOT {
    fn inspect(&self) -> Box<dyn QPublicGraph<Graph>> {
        // TODO
        Box::new(GraphPublicGraph::Basis(QBasis::Zero))
    }
}
impl QInspectable<Graph> for GraphCZ {
    fn inspect(&self) -> Box<dyn QPublicGraph<Graph>> {
        // TODO
        Box::new(GraphPublicGraph::Basis(QBasis::Zero))
    }
}
