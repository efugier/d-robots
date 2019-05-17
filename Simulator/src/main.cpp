#include <QApplication>


int main(int argc, char** argv)
{
    QApplication app(argc, argv);
    QApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    app.setOrganizationName("SR05-project");
    app.setApplicationName("Simulateur");


    return app.exec();
}
