use joylight_backend::parameter::parameter_view::ViewValue;
use joylight_backend::effects;
use joylight_backend::effects::io::NodeDataset;
use joylight_backend::setup_logger;
use smallvec::smallvec;

fn main() {
    setup_logger();

    let input1 = smallvec![ViewValue::F64(100.0)];
    let input2 = smallvec![
        ViewValue::F64(100.0),
        ViewValue::I64(1000),
        ViewValue::F64(2000.0)
    ];

    {
        println!("Node processing test 1");
        let input = NodeDataset {
            packets: smallvec![Some(input1.clone())],
        };
        println!(" Input:  {:?}", input);

        let defn = effects::nodes::math::log();
        let node = defn.build("log");

        let null_parameters = smallvec![];
        let output = (node.definition.processor)(&input, &null_parameters, 1);
        println!(" Output: {:?}", output);
    }

    {
        println!("Node processing test 2");
        let input = NodeDataset {
            packets: smallvec![Some(input1), Some(input2)],
        };
        println!(" Input:  {:?}", input);

        let defn = effects::nodes::math::log();
        let node = defn.build("log");

        let null_parameters = smallvec![];
        let output = (node.definition.processor)(&input, &null_parameters, 1);
        println!(" Output: {:?}", output);
    }

    {
        println!("Node processing test 3");
        let random = effects::nodes::math::random();
        let log = effects::nodes::math::log();
        let abs = effects::nodes::math::abs();

        let mut graph = effects::graph::EffectGraph::new();
        let random1 = graph.add_node(random.build("random1"));
        let log1 = graph.add_node(log.build("log1"));
        let random2 = graph.add_node(random.build("random2"));
        let log2 = graph.add_node(log.build("log2"));
        let abs1 = graph.add_node(abs.build("abs1"));

        effects::node::link(&random1, &log1, 0);
        effects::node::link(&log2, &abs1, 0);
        effects::node::link(&abs1, &log1, 0);
        effects::node::link(&random2, &log2, 0);
        effects::node::link(&random1, &log2, 1);

        let _ = graph.add_node(random.build("random3"));
        let _ = graph.add_node(random.build("random4"));

        graph.process().unwrap();
        println!("{}", graph.graphviz());
    }
}