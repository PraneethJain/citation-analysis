for file in *.txt; do
    if [ -f "$file" ] && [ -r "$file" ]; then
        echo "Processing file: $file"
        filename="${file%.*}"
        displayname=$(echo "$filename" | sed -r 's/_/ /g; s/(^| )([a-z])/\1\u\2/g')
        gnuplot -e "in='${filename}.txt'; out='${filename}.png'; y='${displayname}';" plot.gp
    else
        echo "Error: File '$file' does not exist or is not readable."
    fi
done

