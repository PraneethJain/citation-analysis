set xlabel "Date"
set ylabel "SCC Counts"

set xdata time
set timefmt "%Y-%m-%d"

set terminal png
set output "scc_counts.png"

set xtics 10000000 rotate by -80 font ",8"

plot "scc_counts.txt" using (timecolumn(1, "%Y-%m-%d")):2 with lines notitle

