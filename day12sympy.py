from sympy import And
from sympy import symbols, S, pprint, solveset, Mod, Eq, linsolve, solve
from sympy import simplify
from sympy.solvers.solveset import nonlinsolve
from sympy.solvers.diophantine.diophantine import diophantine

t = symbols('t')

#pprint(solveset((7 + n) and (17 * n + 123), n, S.Naturals))

equations = And(
        Eq(Mod(t, 13), 1), 
        Eq(Mod(t, 7), 0), 
)
pprint(simplify(equations))
