#![allow(dead_code)]
#![allow(unused_variables)]
use std::any::Any;
use std::rc::Rc;

////////// traits //////////

// Graph, ClassicPure, Qisikit, ClassicDensityなどの実装
trait QBackend {}

#[derive(Debug)]

// 測定結果
enum QComputationalBasis {
    Zero,
    One,
}

// グラフに関する操作（複数Qubit）
trait QPublicMultiGraph<T>
where
    T: QBackend,
{
    // 複数Qubitの測定を行う
    fn measure(qs: Vec<Box<dyn Qubit<T>>>) -> Vec<QComputationalBasis>;
}

// グラフに関する操作（単一Qubit）
trait QPublicGraph<T>
where
    T: QBackend,
{
    // 単一Qubitの測定を行う
    fn measure(&self) -> QComputationalBasis;
}

// Qubit
trait Qubit<T>
where
    T: QBackend,
{
    // 単一Qubitの計算依存グラフを作成する
    fn inspect(&self) -> Box<dyn QPublicGraph<T>>;
    // 単一Qubitだけでなく、複数のQubitの計算依存グラフをまとめて作成する（共通する依存関係を適切に処理する）
    fn associate(&self, qs: Vec<Box<dyn Qubit<T>>>) -> Vec<Box<dyn QPublicGraph<T>>>;
}

// 量子ゲート（測定も含む）
trait QGate<T>
where
    T: QBackend,
    Self::Input: std::fmt::Debug,
    Self::Output: std::fmt::Debug,
{
    // 量子ゲートの入力
    type Input;
    // 量子ゲートの出力
    type Output;
    // 量子ゲートの入力から出力の構築
    fn apply(qs: Self::Input) -> Self::Output;
}

////////// Graph Backend //////////

// 計算グラフ
struct Graph {}
impl QBackend for Graph {}

#[derive(Debug)]
struct GraphGateOutput {
    index: u32,
    gate: Rc<dyn Any>,
}

struct GraphPublicGraph {}
impl QPublicGraph<Graph> for GraphPublicGraph {
    fn measure(&self) -> QComputationalBasis {
        QComputationalBasis::Zero
    }
}

#[derive(Debug)]
enum GraphQubit {
    Computational(QComputationalBasis),
    GateOutput(GraphGateOutput),
}
impl Qubit<Graph> for GraphQubit {
    fn inspect(&self) -> Box<dyn QPublicGraph<Graph>> {
        Box::new(GraphPublicGraph {})
    }
    fn associate(&self, qs: Vec<Box<dyn Qubit<Graph>>>) -> Vec<Box<dyn QPublicGraph<Graph>>> {
        vec![Box::new(GraphPublicGraph {})]
    }
}

#[derive(Debug)]
struct GraphHadamard {
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
struct GraphCNOT {
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
struct GraphCZ {
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

////////// main //////////

fn main() {
    // 初期状態を準備（0 -> 2へテレポートする）
    let q0_0 = GraphQubit::Computational(QComputationalBasis::Zero);
    let q1_0 = GraphQubit::Computational(QComputationalBasis::Zero);
    let q2_0 = GraphQubit::Computational(QComputationalBasis::Zero);

    // 1にH
    let q1_1 = GraphHadamard::apply(Box::new(q1_0));

    // 1,2にCNOT
    let (q1_2, q2_2) = GraphCNOT::apply((q1_1, Box::new(q2_0)));

    // 0,1にCNOT
    let (q0_3, q1_3) = GraphCNOT::apply((Box::new(q0_0), q1_2));

    // 0にH
    let q0_4 = GraphHadamard::apply(q0_3);

    // 測定フェーズ: 1,2にCNOT
    let (q1_5, q2_5) = GraphCNOT::apply((q1_3, q2_2));

    // 測定フェーズ: 0,2にCZ
    let (q0_6, q2_6) = GraphCZ::apply((q0_4, q2_5));

    // 最後のqubit
    let (q0_7, q1_7, q2_7) = (q0_6, q1_5, q2_6);

    println!("{:#?}", (q0_7, q1_7, q2_7));
}
