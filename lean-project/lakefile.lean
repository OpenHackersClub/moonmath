import Lake
open Lake DSL

package moonmath where
  leanOptions := #[⟨`autoImplicit, false⟩]

require "leanprover-community" / "mathlib" @ git "v4.28.0"
