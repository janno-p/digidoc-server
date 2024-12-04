#include "digidoc-server/include/digidoc.hpp"
#include <Container.h>
#include <string>

void initialize()
{
    digidoc::initialize();
}

void terminate()
{
    digidoc::terminate();
}

class DigiDoc::impl
{
    friend DigiDoc;

    std::unique_ptr<digidoc::Container> doc;

    impl(rust::Str path)
        : doc(digidoc::Container::openPtr((std::string)path))
    {
    }
};

DigiDoc::DigiDoc(rust::Str path)
    : impl(new class DigiDoc::impl(path))
{
}

std::unique_ptr<DigiDoc> container_open_ptr(rust::Str path)
{
    return std::unique_ptr<DigiDoc>(new DigiDoc(path));
}
