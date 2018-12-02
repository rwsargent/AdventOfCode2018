/*
**
** The C code is generated by [ATS/Postiats-0-3-12]
** The starting compilation time is: 2018-12-1: 22h:54m
**
*/

/*
** include runtime header files
*/
#ifndef _ATS_CCOMP_HEADER_NONE_
#include "pats_ccomp_config.h"
#include "pats_ccomp_basics.h"
#include "pats_ccomp_typedefs.h"
#include "pats_ccomp_instrset.h"
#include "pats_ccomp_memalloc.h"
#ifndef _ATS_CCOMP_EXCEPTION_NONE_
#include "pats_ccomp_memalloca.h"
#include "pats_ccomp_exception.h"
#endif // end of [_ATS_CCOMP_EXCEPTION_NONE_]
#endif /* _ATS_CCOMP_HEADER_NONE_ */


/*
** include prelude cats files
*/
#ifndef _ATS_CCOMP_PRELUDE_NONE_
//
#include "prelude/CATS/basics.cats"
#include "prelude/CATS/integer.cats"
#include "prelude/CATS/pointer.cats"
#include "prelude/CATS/integer_long.cats"
#include "prelude/CATS/integer_size.cats"
#include "prelude/CATS/integer_short.cats"
#include "prelude/CATS/bool.cats"
#include "prelude/CATS/char.cats"
#include "prelude/CATS/float.cats"
#include "prelude/CATS/integer_ptr.cats"
#include "prelude/CATS/integer_fixed.cats"
#include "prelude/CATS/memory.cats"
#include "prelude/CATS/string.cats"
#include "prelude/CATS/strptr.cats"
//
#include "prelude/CATS/fprintf.cats"
//
#include "prelude/CATS/filebas.cats"
//
#include "prelude/CATS/list.cats"
#include "prelude/CATS/option.cats"
#include "prelude/CATS/array.cats"
#include "prelude/CATS/arrayptr.cats"
#include "prelude/CATS/arrayref.cats"
#include "prelude/CATS/matrix.cats"
#include "prelude/CATS/matrixptr.cats"
//
#endif /* _ATS_CCOMP_PRELUDE_NONE_ */
/*
** for user-supplied prelude
*/
#ifdef _ATS_CCOMP_PRELUDE_USER_
//
#include _ATS_CCOMP_PRELUDE_USER_
//
#endif /* _ATS_CCOMP_PRELUDE_USER_ */
/*
** for user2-supplied prelude
*/
#ifdef _ATS_CCOMP_PRELUDE_USER2_
//
#include _ATS_CCOMP_PRELUDE_USER2_
//
#endif /* _ATS_CCOMP_PRELUDE_USER2_ */

/*
staload-prologues(beg)
*/
/*
staload-prologues(end)
*/
/*
typedefs-for-tyrecs-and-tysums(beg)
*/
/*
typedefs-for-tyrecs-and-tysums(end)
*/
/*
dynconlst-declaration(beg)
*/
/*
dynconlst-declaration(end)
*/
/*
dyncstlst-declaration(beg)
*/
ATSdyncst_mac(atspre_print_double)
ATSdyncst_mac(atspre_print_newline)
/*
dyncstlst-declaration(end)
*/
/*
dynvalist-implementation(beg)
*/
/*
dynvalist-implementation(end)
*/
/*
exnconlst-declaration(beg)
*/
#ifndef _ATS_CCOMP_EXCEPTION_NONE_
ATSextern()
atsvoid_t0ype
the_atsexncon_initize
(
  atstype_exnconptr d2c, atstype_string exnmsg
) ;
#endif // end of [_ATS_CCOMP_EXCEPTION_NONE_]
/*
exnconlst-declaration(end)
*/
/*
extypelst-declaration(beg)
*/
/*
extypelst-declaration(end)
*/
/*
assumelst-declaration(beg)
*/
#ifndef _ATS_CCOMP_ASSUME_CHECK_NONE_
#endif // #ifndef(_ATS_CCOMP_ASSUME_CHECK_NONE_)
/*
assumelst-declaration(end)
*/
#if(0)
ATSextern()
atsvoid_t0ype
mainats_void_0() ;
#endif // end of [QUALIFIED]

/*
/Users/dlewis/personal/AdventOfCode2018/david-ATS/src/hello.dats: 17(line=1, offs=17) -- 36(line=1, offs=36)
*/
/*
local: 
global: mainats_void_0$0$0(level=0)
local: 
global: 
*/
ATSextern()
atsvoid_t0ype
mainats_void_0()
{
/* tmpvardeclst(beg) */
// ATStmpdec_void(tmpret0) ;
// ATStmpdec_void(tmp1) ;
/* tmpvardeclst(end) */
ATSfunbody_beg()
/*
emit_instr: loc0 = /Users/dlewis/personal/AdventOfCode2018/david-ATS/src/hello.dats: 11(line=1, offs=11) -- 36(line=1, offs=36)
*/
ATSINSflab(__patsflab_main_void_0):
/*
emit_instr: loc0 = /Users/dlewis/personal/AdventOfCode2018/david-ATS/src/hello.dats: 22(line=1, offs=22) -- 36(line=1, offs=36)
*/
ATSINSmove_void(tmp1, atspre_print_double(ATSPMVf0loat(2.718))) ;

/*
emit_instr: loc0 = /Users/dlewis/personal/AdventOfCode2018/david-ATS/src/hello.dats: 22(line=1, offs=22) -- 36(line=1, offs=36)
*/
ATSINSmove_void(tmpret0, atspre_print_newline()) ;

ATSfunbody_end()
ATSreturn_void(tmpret0) ;
} /* end of [mainats_void_0] */

/*
** for initialization(dynloading)
*/
ATSdynloadflag_minit(_057_Users_057_dlewis_057_personal_057_AdventOfCode2018_057_david_055_ATS_057_src_057_hello_056_dats__dynloadflag) ;
ATSextern()
atsvoid_t0ype
_057_Users_057_dlewis_057_personal_057_AdventOfCode2018_057_david_055_ATS_057_src_057_hello_056_dats__dynload()
{
ATSfunbody_beg()
ATSdynload(/*void*/)
ATSdynloadflag_sta(
_057_Users_057_dlewis_057_personal_057_AdventOfCode2018_057_david_055_ATS_057_src_057_hello_056_dats__dynloadflag
) ;
ATSif(
ATSCKiseqz(
_057_Users_057_dlewis_057_personal_057_AdventOfCode2018_057_david_055_ATS_057_src_057_hello_056_dats__dynloadflag
)
) ATSthen() {
ATSdynloadset(_057_Users_057_dlewis_057_personal_057_AdventOfCode2018_057_david_055_ATS_057_src_057_hello_056_dats__dynloadflag) ;
/*
dynexnlst-initize(beg)
*/
/*
dynexnlst-initize(end)
*/
} /* ATSendif */
ATSfunbody_end()
ATSreturn_void(tmpret_void) ;
} /* end of [*_dynload] */

/*
** the ATS runtime
*/
#ifndef _ATS_CCOMP_RUNTIME_NONE_
#include "pats_ccomp_runtime.c"
#include "pats_ccomp_runtime_memalloc.c"
#ifndef _ATS_CCOMP_EXCEPTION_NONE_
#include "pats_ccomp_runtime2_dats.c"
#ifndef _ATS_CCOMP_RUNTIME_TRYWITH_NONE_
#include "pats_ccomp_runtime_trywith.c"
#endif /* _ATS_CCOMP_RUNTIME_TRYWITH_NONE_ */
#endif // end of [_ATS_CCOMP_EXCEPTION_NONE_]
#endif /* _ATS_CCOMP_RUNTIME_NONE_ */

/*
** the [main] implementation
*/
int
main
(
int argc, char **argv, char **envp
) {
int err = 0 ;
_057_Users_057_dlewis_057_personal_057_AdventOfCode2018_057_david_055_ATS_057_src_057_hello_056_dats__dynload() ;
ATSmainats_void_0(err) ;
return (err) ;
} /* end of [main] */

/* ****** ****** */

/* end-of-compilation-unit */
