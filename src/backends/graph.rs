use crate::core::*;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;
use uuid::Uuid;

// バックエンド宣言
// 計算グラフ
pub struct Graph {}
impl QBackend for Graph {}

trait GraphQGate: QGate<Graph> {}

// 中身にはアクセスできないのでセーフ
#[derive(Debug)]
pub struct GraphGateOutput {
    index: u32,
    gate: Rc<dyn QInspectable<Graph>>,
}

#[derive(Debug)]
struct IdWrapper<T>
where
    T: QInspectable<Graph>,
{
    id: Option<Uuid>,
    nodes: HashSet<Rc<dyn QInspectable<Graph>>>,
    content: T,
}

// 計算グラフの公開された表現
#[derive(Debug)]
pub enum GraphPublicGraph {
    Gate {
        gate_name: String,
        input: Vec<Rc<dyn QPublicGraph<Graph>>>,
    },
    GateOutput {
        index: u32,
        gate: Rc<dyn QPublicGraph<Graph>>,
    },
    // basisにnodesはいらない
    Basis(QBasis),
}
impl QPublicGraph<Graph> for GraphPublicGraph {
    // Graphのmeasureを直接実行しても計算は行われないのでこれで良い
    fn measure(&self) -> QBasis {
        panic!("not implemented");
    }
}

// Qubit
#[derive(Debug)]
pub enum GraphQubit {
    Basis(QBasis),
    Gate(GraphGateOutput),
}
impl Qubit<Graph> for GraphQubit {}

pub fn init_graph_qubit(b: QBasis) -> Box<dyn Qubit<Graph>> {
    Box::new(GraphQubit::Basis(b))
}

// H ゲート
#[derive(Debug)]
pub struct GraphHadamard {
    input: Rc<<GraphHadamard as QGate<Graph>>::Input>,
}
impl QGate<Graph> for GraphHadamard {
    type Input = Box<dyn Qubit<Graph>>;
    type Output = Box<dyn Qubit<Graph>>;
    fn apply(q: Self::Input) -> Self::Output {
        let gate = Rc::new(GraphHadamard { input: Rc::new(q) });
        let gate_output = GraphGateOutput {
            index: 0,
            gate: gate.clone(),
        };
        Box::new(GraphQubit::Gate(gate_output))
    }
}

// CNOT ゲート
#[derive(Debug)]
pub struct GraphCNOT {
    input: Rc<<GraphCNOT as QGate<Graph>>::Input>,
}
impl QGate<Graph> for GraphCNOT {
    type Input = (Box<dyn Qubit<Graph>>, Box<dyn Qubit<Graph>>);
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
    type Input = (Box<dyn Qubit<Graph>>, Box<dyn Qubit<Graph>>);
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
    fn inspect(&self) -> Rc<dyn QPublicGraph<Graph>> {
        match self {
            GraphQubit::Basis(QBasis::Zero) => Rc::new(GraphPublicGraph::Basis(QBasis::Zero)),
            GraphQubit::Basis(QBasis::One) => Rc::new(GraphPublicGraph::Basis(QBasis::One)),
            GraphQubit::Gate(gate_output) => gate_output.gate.inspect(),
        }
    }
}
impl QInspectable<Graph> for GraphHadamard {
    fn inspect(&self) -> Rc<dyn QPublicGraph<Graph>> {
        Rc::new(GraphPublicGraph::Gate {
            gate_name: "GraphHadamard".to_string(),
            input: vec![self.input.inspect()],
        })
    }
}
impl QInspectable<Graph> for GraphCNOT {
    fn inspect(&self) -> Rc<dyn QPublicGraph<Graph>> {
        Rc::new(GraphPublicGraph::Gate {
            gate_name: "GraphCNOT".to_string(),
            input: vec![self.input.0.inspect(), self.input.1.inspect()],
        })
    }
}
impl QInspectable<Graph> for GraphCZ {
    fn inspect(&self) -> Rc<dyn QPublicGraph<Graph>> {
        Rc::new(GraphPublicGraph::Gate {
            gate_name: "GraphCZ".to_string(),
            input: vec![self.input.0.inspect(), self.input.1.inspect()],
        })
    }
}
