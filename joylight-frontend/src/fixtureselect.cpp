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


    ConnectToSocket(socket);

    while (socket.pendingDatagramSize() <= 0 ) {
        std::cerr << "Quering fixture data..." << std::endl;
        socket.writeDatagram("Hello\n", QHostAddress(address), 7096);
        QThread::msleep(100);
    }

    QByteArray datagram;
    datagram.resize(socket.pendingDatagramSize());
    socket.readDatagram(datagram.data(), datagram.size());

    auto json = datagram.toStdString();
    std::cout << "Received JSON data: " << json << std::endl;

    auto j = nlohmann::json::parse(json);

    // mmm very interesting
    std::cout << "Interesting light statistics:" << std::endl;
    std::cout << "  Number of lights: " << j.size() << std::endl;
    std::cout << "  Light names: ";

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
