for ((i=1992; i<=2002; i++)); do
    for ((j=1; j<=12; j++)); do
        padded_j=$(printf "%02d" $j)
        input_file="g${i}-${padded_j}.gv"
        output_file="g${i}-${padded_j}.png"
        
        sfdp -Goverlap=prism -Gsize=10,10\! -Tpng "$input_file" -o "$output_file"
        
        if [ $? -eq 0 ]; then
            echo "File '$output_file' generated successfully."
        else
            echo "Error generating file '$output_file'."
        fi
    done
done
