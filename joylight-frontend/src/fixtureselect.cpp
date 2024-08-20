#include "fixtureselect.h"
#include "ui_fixtureselect.h"
#include <iostream>

#include <QApplication>
#include <QThread>
#include <QFile>

#include "json.hpp"
#include "config.h"
#include "socket.h"

FixtureSelect::FixtureSelect(QWidget *parent)
    : QDialog(parent)
    , ui(new Ui::FixtureSelect)
{
    auto socket = ConnectToSocket();

    // TODO error handling
    try {
        socket.send(zmq::str_buffer("Give me your fixtures please! Thanks in advance."));
        zmq::message_t response;
        (void) socket.recv(response);

        auto json = response.to_string();
        std::cout << "Received JSON data: " << json << std::endl;

        auto j = nlohmann::json::parse(json);

        std::cout << "Interesting light statistics:" << std::endl;
        std::cout << "  Number of lights: " << j["fixtures"].size() << std::endl;
        std::cout << "  Light names: ";
    }
    catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
    }

    ui->setupUi(this);

}

void FixtureSelect::selectionChanged(QListWidgetItem *current, QListWidgetItem *previous) {

    qDebug() << "Selection chagned" << (current->text());
}

void FixtureSelect::accept() {
    if (ui->fixtureNameEdit->text() == "") {
        ui->errorMessage->setText("Fixture Name can not be empty");
        return;
    }

    emit selectedFixture(Fixture{
        .fixture_name = ui->fixtureNameEdit->text(),
    });

    // TODO: Send selection to backend and verify that it is valid
    // - no duplicate names
    // - valid universe (enough parameters/exists?)
    
    this->close();
}


FixtureSelect::~FixtureSelect()
{
    delete ui;
}
