INPUT = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"

×(x, y) = x + y
⊕(x, y) = x * y

INPUT = replace(INPUT, "*" => "⊕")
INPUT = replace(INPUT, "+" => "×")
exec = eval ∘ Meta.parse
@show sum(exec.(split(INPUT, "\n")))
