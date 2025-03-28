use std::sync::{Arc, RwLock};
use std::time::Instant;

use joylight_backend::effects::io::NodeDataset;
use joylight_backend::effects::node::link;
use joylight_backend::effects::nodes::output::output;
use joylight_backend::fixtures::{Filter, FixtureRef};
use joylight_backend::parameters::parameter_view::ViewValue;
use joylight_backend::show::Show;
use joylight_backend::{effects, setup_logger};
use log::*;
use smallvec::smallvec;

fn main() {
    setup_logger();

    let input1 = smallvec![ViewValue::F64(100.0)];
    let input2 = smallvec![ViewValue::F64(100.0), ViewValue::I64(1000), ViewValue::F64(2000.0)];

    {
        println!("Node processing test 1");
        let input = NodeDataset {
            packets: smallvec![input1.clone()],
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
            packets: smallvec![input1, input2],
        };
        println!(" Input:  {:?}", input);

        let defn = effects::nodes::math::log();
        let node = defn.build("log");

        let null_parameters = smallvec![];
        let output = (node.definition.processor)(&input, &null_parameters, 1);
        println!(" Output: {:?}", output);
    }

    let random = effects::nodes::math::random();
    let clock = effects::nodes::math::clock(Instant::now());
    let log = effects::nodes::math::log();
    let abs = effects::nodes::math::abs();

    {
        println!("Node processing test 3");

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

    {
        println!("Node processing test 4");

        let brightness = joylight_backend::parameters::parameter_type::ParameterType::new(
            "brightness",
            "Brightness",
            Box::new(joylight_backend::parameters::parameter_view::percentage()),
            Box::new(
                joylight_backend::parameters::parameter_encoding::DMXMappingTransformer {
                    input_min: 0.0,
                    input_max: 100.0,
                    size: 1,
                    endianness: joylight_backend::parameters::parameter_encoding::Endianness::Big,
                },
            ),
            joylight_backend::parameters::parameter_value::ParameterDescription::Number(1),
            joylight_backend::parameters::parameter_value::ParameterValue::Number(vec![0.0]),
            None,
        );

        let fixtemp = joylight_backend::fixtures::fixture_template::FixtureTemplate {
            name: "Dimmer".to_string(),
            parameters: vec![brightness.clone()],
        };

        let fixture1 = FixtureRef::new_from_move(joylight_backend::fixtures::Fixture::new("Dimmer1", &fixtemp));
        let fixture2 = FixtureRef::new_from_move(joylight_backend::fixtures::Fixture::new("Dimmer2", &fixtemp));
        let fixture3 = FixtureRef::new_from_move(joylight_backend::fixtures::Fixture::new("Dimmer3", &fixtemp));

        let mut show = Show::default();
        show.add_fixture(fixture1);
        show.add_fixture(fixture2);
        show.add_fixture(fixture3);

        let selection = Arc::new(RwLock::new(
            joylight_backend::fixtures::selection::FilteredSelection::new(
                "all",
                Filter::Predicate(Box::new(|f| f.read(|f| f.name == "Dimmer2").unwrap_or(false))),
            ),
        ));

        show.add_selection(selection.clone());
        show.refresh_fixtures();

        let output = output("brightness", selection);

        let mut graph: effects::graph::EffectGraph<'_> = effects::graph::EffectGraph::new();
        let node1 = graph.add_node(clock.build("input"));
        let node2 = graph.add_node(output.build("output"));
        link(&node1, &node2, 0);

        let dbg = || {
            info!(
                "Fixture brightnesses: {:?}",
                show.fixtures
                    .iter()
                    .map(|f| f.1.read(|f| f.get_parameter_values().clone()))
                    .collect::<Vec<_>>()
            )
        };

        dbg();
        for _ in 0..2 {
            graph.process().unwrap();
            dbg();
        }
    }
}
