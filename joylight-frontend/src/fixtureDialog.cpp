#include "fixtureDialog.h"
#include "ui_fixtureDialog.h"
#include <iostream>
#include "fixtureselect.h"

#include <QApplication>
#include <QThread>
#include <QFile>

#include "json.hpp"
#include "config.h"
#include "socket.h"

FixtureDialog::FixtureDialog(QWidget *parent)
    : QDialog(parent)
    , ui(new Ui::FixtureDialog)
{

    ui->setupUi(this);
}

FixtureDialog::~FixtureDialog()
{
    delete ui;
}

void FixtureDialog::openAddFixtureDialog() {
    FixtureSelect fixtureSelectUi(this);
    connect(&fixtureSelectUi, SIGNAL(selectedFixture(Fixture)), this, SLOT(addFixture(Fixture)));

    qDebug() << "Show fixture patching:";

    fixtureSelectUi.setModal(true);
    fixtureSelectUi.exec();
}

void FixtureDialog::addFixture(Fixture fixture) {
    qDebug() << "Add fixture";
}
