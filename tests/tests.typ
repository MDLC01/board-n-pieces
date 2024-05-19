#assert(
  "lib" in sys.inputs.keys(),
  message: "the path to the library root should be passed as `--input lib=<path>`",
)

// Tests to the package's public interface.
#include "api.typ"

// Tests to the logic.
#include "logic.typ"
