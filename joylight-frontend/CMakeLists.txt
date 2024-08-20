cmake_minimum_required(VERSION 3.16)

project(joylight-frontend VERSION 0.1 LANGUAGES CXX)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC_SEARCH_PATHS ${CMAKE_CURRENT_SOURCE_DIR}/style/forms)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/inc
)

find_library(ZeroMQ czmq zmq libzmq REQUIRED)
find_package(QT NAMES Qt6 Qt5 REQUIRED COMPONENTS Widgets Network Core Gui)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Widgets Network Core Gui)

set(PROJECT_SOURCES
        main.cpp
        src/mainwindow.cpp
        src/fixtureselect.cpp
        src/fixtureDialog.cpp
        src/socket.cpp
        inc/mainwindow.h
        inc/config.h
        inc/fixture.h
        inc/fixtureselect.h
        inc/fixtureDialog.h
        inc/json.hpp
        inc/socket.h
        style/forms/fixtureDialog.ui
        style/forms/fixtureselect.ui
        style/forms/mainwindow.ui
)

if(${QT_VERSION_MAJOR} GREATER_EQUAL 6)
    qt_add_executable(joylight-frontend
        MANUAL_FINALIZATION
        ${PROJECT_SOURCES}
    )
else()
    add_executable(joylight-frontend
        ${PROJECT_SOURCES}
    )
endif()

target_link_libraries(joylight-frontend PRIVATE 
    ${ZeroMQ}
    Qt${QT_VERSION_MAJOR}::Widgets
    Qt${QT_VERSION_MAJOR}::Network
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
)

# Qt for iOS sets MACOSX_BUNDLE_GUI_IDENTIFIER automatically since Qt 6.1.
# If you are developing for iOS or macOS you should consider setting an
# explicit, fixed bundle identifier manually though.
if(${QT_VERSION} VERSION_LESS 6.1.0)
  set(BUNDLE_ID_OPTION MACOSX_BUNDLE_GUI_IDENTIFIER com.example.joylight-frontend)
endif()
set_target_properties(joylight-frontend PROPERTIES
    ${BUNDLE_ID_OPTION}
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

include(GNUInstallDirs)
install(TARGETS joylight-frontend
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

if(QT_VERSION_MAJOR EQUAL 6)
    qt_finalize_executable(joylight-frontend)
endif()
