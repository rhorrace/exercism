raindrops <- function(number) {
  raindrop <- ''
  if (mod(number, 3) == 0) {
    raindrop <- paste(raindrop, 'Pling', sep='')
  }
  if (mod(number, 5) == 0) {
    raindrop <- paste(raindrop, 'Plang', sep='')
  }
  if (mod(number, 7) == 0) {
    raindrop <- paste(raindrop, 'Plong', sep='')
  }
  
  ifelse(raindrop == '', paste(number), raindrop)
}

mod <- function(n, m) {n - m * floor(n/m)}