leap <- function(year) {
  (mod(year, 4) == 0 && mod(year, 100) != 0) || mod(year, 400) == 0
}

mod <- function(n, m) {n - m * floor(n/m)}