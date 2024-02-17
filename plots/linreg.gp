data1_file = "wcc_size_linreg.txt"
set xlabel "Date"
set xtics rotate by -45 nomirror
set xdata time
set timefmt "%Y-%m-%d"


data2_file = "largest_wcc_sizes.txt"

set title "Linear Regression Results"
set key top left

set terminal png
set output "wcc_size_linreg.png"

plot data1_file using 1:2 with lines title "Prediction", \
     data2_file using 1:2 with lines title "Actual"

