#ifndef FIXTURESELECT_H
#define FIXTURESELECT_H

#include <iostream>

#include "fixture.h"

#include <QUdpSocket>
#include <QDialog>
#include <QListWidget>

namespace Ui {
class FixtureSelect;
}

class FixtureSelect : public QDialog
{
    Q_OBJECT

public:
    explicit FixtureSelect(QWidget *parent = nullptr);
    virtual ~FixtureSelect();

private:
    Ui::FixtureSelect *ui;

    QUdpSocket socket;

public slots:
    void selectionChanged(QListWidgetItem *current, QListWidgetItem *previous);
    void accept();
signals:
    void selectedFixture(Fixture);
};

#endif // FIXTURESELECT_H
