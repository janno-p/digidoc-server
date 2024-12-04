#ifndef DIGIDOC_HPP_INCLUDED
#define DIGIDOC_HPP_INCLUDED

#include "rust/cxx.h"
#include <memory>
#include <Exception.h>

class DigiDoc
{
public:
    DigiDoc(rust::Str path);

private:
    class impl;
    std::shared_ptr<impl> impl;
};

void initialize();
void terminate();

std::unique_ptr<DigiDoc> container_open_ptr(rust::Str path);

namespace rust
{
    namespace behavior
    {
        template <typename Try, typename Fail>
        static void trycatch(Try &&func, Fail &&fail) noexcept try {
            func();
        } catch (const digidoc::Exception &e) {
            fail(e.msg());
        }
    }
}

#endif
