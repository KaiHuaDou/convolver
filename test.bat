cargo rr -- luva input.png output-blur.png -f 3-blur
cargo rr -- luva input.png output-dog.png -f 7-dog-3-1-0
cargo rr -- luva input.png output-sharpen.png -f 3-sharpen
cargo rr -- luva input.png output-emboss.png -f 3-emboss-se
cargo rr -- luva input.png output-gauss-blur.png -f 5-gauss-blur-1
cargo rr -- luva input.png output-gauss-sharpen.png -f 5-gauss-sharpen-1
cargo rr -- luva input.png output-kirsch.png -f 3-kirsch_ne
cargo rr -- luva input.png output-laplacian_4.png -f 3-laplacian_4
cargo rr -- luva input.png output-laplacian_8.png -f 3-laplacian_8
cargo rr -- luva input.png output-laplacian_8r.png -f 3-laplacian_8r
cargo rr -- luva input.png output-laplacian_og.png -f 5-laplacian_og
cargo rr -- luva input.png output-max.png -f 3-max
cargo rr -- luva input.png output-median.png -f 3-median
cargo rr -- luva input.png output-min.png -f 3-min
cargo rr -- luva input.png output-motion.png -f 31-motion-31-135
cargo rr -- luva input.png output-prewitt-h.png -f 3-prewitt_h
cargo rr -- luva input.png output-prewitt-v.png -f 3-prewitt_v
cargo rr -- luva input.png output-robinson.png -f 3-robinson_ne
cargo rr -- luva input.png output-scharr-h.png -f 3-scharr_h
cargo rr -- luva input.png output-scharr-v.png -f 3-scharr_v
cargo rr -- luva input.png output-sobel-h.png -f 3-sobel_h
cargo rr -- luva input.png output-sobel-v.png -f 3-sobel_v
cargo rr -- luva input.png output-unsharp_masking.png -f 3-unsharp_masking
cargo rr -- luva input.png output.png -f 3-none
cargo rr -- add input.png output-emboss.png output-emboss-m.png
cargo rr -- add output-laplacian_8r.png output-laplacian_8.png output-laplacian_8m.png
cargo rr -- add output-prewitt-h.png output-prewitt-v.png output-prewitt-m.png
cargo rr -- add output-scharr-h.png output-scharr-v.png output-scharr-m.png
cargo rr -- add output-sobel-h.png output-sobel-v.png output-sobel-m.png
