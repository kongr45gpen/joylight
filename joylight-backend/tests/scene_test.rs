use joylight_backend::fixtures::common_fixtures::DIMMER;
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

    let fixture = FixtureRef::new_from_move(Fixture::new("SingleFixture", &DIMMER));

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
