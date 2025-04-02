use joylight_backend::fixtures::{Fixture, FixtureRef, FixtureTemplate};
use joylight_backend::parameters::{
    ParameterType, ParameterValue, ParameterValueDescription, ViewValue, parameter_encoding, parameter_view,
};
use joylight_backend::setup_logger;
use joylight_backend::show::{BlendingMode, Scene};
use smallvec::smallvec;

#[test]
fn scene_for_single_fixture() {
    setup_logger();

    let brightness = ParameterType::new(
        "brightness",
        "Brightness",
        Box::new(parameter_view::percentage()),
        Box::new(parameter_encoding::DMXMappingTransformer {
            input_min: 0.0,
            input_max: 100.0,
            size: 1,
            endianness: parameter_encoding::Endianness::Big,
        }),
        ParameterValueDescription::Number(1),
        ParameterValue::Number(vec![0.0]),
        None,
    );

    let fixture_template = FixtureTemplate {
        name: "SingleFixture".to_string(),
        parameters: vec![brightness.clone()],
    };

    let fixture = FixtureRef::new_from_move(Fixture::new("SingleFixture", &fixture_template));

    let mut scene = Scene::default();

    let param_0_value = || {
        fixture
            .read(|f| f.get_parameter_by_number(0).unwrap().value.clone())
            .unwrap()
    };

    assert_eq!(param_0_value(), ParameterValue::Number(vec![0.0]));

    scene.set_parameter(
        fixture.clone(),
        0,
        smallvec![ViewValue::F64(50.0)],
        Box::new(parameter_view::percentage()),
    );

    assert_eq!(param_0_value(), ParameterValue::Number(vec![0.0]));
    scene.add_parameter_updates();

    assert_eq!(param_0_value(), ParameterValue::Number(vec![0.0]));

    fixture.write(|f| f.update_parameters()).unwrap();
    assert_eq!(param_0_value(), ParameterValue::Number(vec![50.0]));

    scene.clear();
    fixture.write(|f| f.update_parameters()).unwrap();
    assert_eq!(param_0_value(), ParameterValue::Number(vec![0.0]));
}
