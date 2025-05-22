cargo r --release -- c initial.png result-blur.png -f 3-blur
cargo r --release -- c initial.png result-gauss-blur.png -f 5-gauss-blur-1
cargo r --release -- c initial.png result-gauss-sharpen.png -f 5-gauss-sharpen-1
cargo r --release -- c initial.png result-laplacian_4.png -f 3-laplacian_4
cargo r --release -- c initial.png result-laplacian_8.png -f 3-laplacian_8
cargo r --release -- c initial.png result-laplacian_8r.png -f 3-laplacian_8r
cargo r --release -- c initial.png result-laplacian_m.png -f 5-laplacian_m
cargo r --release -- c initial.png result-motion.png -f 31-motion-31-135
cargo r --release -- c initial.png result-prewitt-h.png -f 3-prewitt_h
cargo r --release -- c initial.png result-prewitt-v.png -f 3-prewitt_v
cargo r --release -- c initial.png result-scharr-h.png -f 3-scharr_h
cargo r --release -- c initial.png result-scharr-v.png -f 3-scharr_v
cargo r --release -- c initial.png result-sobel-h.png -f 3-sobel_h
cargo r --release -- c initial.png result-sobel-v.png -f 3-sobel_v
cargo r --release -- c initial.png result.png -f 3-none
cargo r --release -- s result-laplacian_8r.png result-laplacian_8.png result-laplacian_8m.png
cargo r --release -- s result-prewitt-h.png result-prewitt-v.png result-prewitt-m.png
cargo r --release -- s result-scharr-h.png result-scharr-v.png result-scharr-m.png
cargo r --release -- s result-sobel-h.png result-sobel-v.png result-sobel-m.png
