import A from "";
export { A }
const AA = A;

// With Import attributes
import B from "" with { type: "json" };
type BB = B;

// No reference
import C from "";

import type D from "";
export type { D };
