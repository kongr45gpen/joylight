#include <iostream>


#include "socket.h"
#include "config.h"

// TODO: Should socket be global?
void ConnectToSocket(QUdpSocket &socket){

    if (!socket.bind(QHostAddress::AnyIPv4, 0, QUdpSocket::ShareAddress)) {
        std::cerr << "Failed to bind to port 7096" << std::endl;
        return;
    } else {
        std::cerr << "Bound to local port " << socket.localPort() << std::endl;
    }

    socket.joinMulticastGroup(QHostAddress(address));
}
