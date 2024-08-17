#include "mainwindow.h"
#include "ui_mainwindow.h"

#include "fixtureselect.h"
#include "fixtureDialog.h"

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);


}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::setText(QString text)
{
    // ui->label->setText(text);
}

void MainWindow::showFixturePatching() {
    FixtureDialog fixtureDialogUi(this);

    qDebug() << "Show fixture patching:";

    fixtureDialogUi.setModal(true);
    fixtureDialogUi.exec();
}


void MainWindow::addFixture(Fixture fixture){
    // qDebug() << "Fixture added: " << fixture.fixture_name << fixture.universe_name;

    // add to cached map
    // fixture_attribute[fixture.fixture_name] = fixture;

    // QTreeWidgetItem *widgetItem = new QTreeWidgetItem();
    // widgetItem->setText(0, fixture.fixture_name);

    // ui->fixtureTreeWidget->addTopLevelItem(widgetItem);

    // widgetItem = new QTreeWidgetItem(widgetItem);
    // widgetItem->setText(0, "Universe");
}
