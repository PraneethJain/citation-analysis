for ((i=1992; i<=1992; i++)); do
    input_file="g${i}.gv"
    output_file="g${i}.png"
    
    sfdp -Goverlap=prism -Gsize=20,20\! -Tpng "$input_file" -o "$output_file"
    
    if [ $? -eq 0 ]; then
        echo "File '$output_file' generated successfully."
    else
      echo "Error generating file '$output_file'."
    fi
done
