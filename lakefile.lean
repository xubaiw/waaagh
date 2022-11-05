import Lake
open Lake DSL

package waaagh 

lean_lib Waaagh

@[default_target]
lean_exe waaagh {
  root := `Main
}
