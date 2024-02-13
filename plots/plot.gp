set xlabel "Date"
set ylabel y

set xdata time
set timefmt "%Y-%m-%d"

set terminal png
set output out

set xtics 10000000 rotate by -80 font ",8"

plot in using (timecolumn(1, "%Y-%m-%d")):2 with lines notitle

