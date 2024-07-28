#include "mainwindow.h"

#include <iostream>

#include <QApplication>
#include <QUdpSocket>
#include <QThread>

#include "json.hpp"

int main(int argc, char *argv[])
{
    QUdpSocket socket;
    auto address = QStringLiteral("127.0.0.1");
    if (!socket.bind(QHostAddress::AnyIPv4, 0, QUdpSocket::ShareAddress)) {
        std::cerr << "Failed to bind to port 7096" << std::endl;
        return 1;
    } else {
        std::cerr << "Bound to local port " << socket.localPort() << std::endl;
    }
    socket.joinMulticastGroup(QHostAddress(address));
    
    while (socket.pendingDatagramSize() <= 0 ) {
        std::cerr << "Waiting for data..." << std::endl;
        socket.writeDatagram("Hello\n", QHostAddress(address), 7096);
        QThread::msleep(100);
    }
    QByteArray datagram;
    datagram.resize(socket.pendingDatagramSize());
    socket.readDatagram(datagram.data(), datagram.size());

    auto json = datagram.toStdString();
    std::cout << "Received JSON data: " << json << std::endl;

    auto j = nlohmann::json::parse(json);

    std::cout << "Interesting light statistics:" << std::endl;
    std::cout << "  Number of lights: " << j.size() << std::endl;
    std::cout << "  Light names: ";
    for (auto& light : j) {
        std::cout << light["name"] << ", ";
    }
    std::cout << std::endl;

    QApplication a(argc, argv);
    MainWindow w;
    w.setText(QString::fromStdString(j.dump(4)));
    w.show();
    return a.exec();
}
