#include "mainwindow.h"

#include <iostream>

#include <QApplication>
#include <QFile>


int main(int argc, char *argv[])
{

    QApplication qapp(argc, argv);

    QFont font("Montserrat");
    font.setPixelSize(12);
    font.setLetterSpacing(QFont::AbsoluteSpacing, 0.25);
    font.setStyleHint(QFont::SansSerif);
    qapp.setFont(font);

    // Load an application style
    QFile styleSheet( "stylesheet.qss" );

    if (styleSheet.open(QIODevice::Text | QIODevice::Unbuffered | QIODevice::ReadOnly)) {
        qapp.setStyleSheet(styleSheet.readAll());
        styleSheet.close();
    } else{
        std::cout << "Can not open stylesheet" << std::endl;
    }

    MainWindow w;

    w.show();
    return qapp.exec();
}
