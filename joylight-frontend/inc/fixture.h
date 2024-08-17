#ifndef FIXTURE_H
#define FIXTURE_H

#include <QString>
#include <cstdint>

enum class FixtureInterface : uint8_t {
    Dmx,
    Mqtt,
    Serial
};

enum class FixtureMode : uint8_t {
    Normal,
    Extended
};

struct Fixture
{
    QString fixture_name{""};
    QString fixture_type{""};
    uint8_t universe_name{0};
    uint8_t patch{0};
    FixtureInterface interface{FixtureInterface::Dmx};
    FixtureMode mode{FixtureMode::Normal};
};

#endif // FIXTURE_H
