cargo run --release -- c -k none -o result.png
cargo run --release -- c -k blur -o result-blur.png
cargo run --release -- c -k canny_sobel_h -o result-sobel-h.png
cargo run --release -- c -k canny_sobel_v -o result-sobel-v.png
cargo run --release -- c -k canny_prewitt_v -o result-prewitt-v.png
cargo run --release -- c -k canny_prewitt_h -o result-prewitt-h.png
cargo run --release -- c -k canny_scharr_h -o result-scharr-h.png
cargo run --release -- c -k canny_scharr_v -o result-scharr-v.png
cargo run --release -- c -k canny_laplacian_4 -o result-laplacian_4.png
cargo run --release -- c -k canny_laplacian_8 -o result-laplacian_8.png
cargo run --release -- c -k canny_laplacian_8r -o result-laplacian_8r.png
cargo run --release -- s result-scharr-h.png result-scharr-v.png result-scharr-m.png
cargo run --release -- s result-prewitt-h.png result-prewitt-v.png result-prewitt-m.png
cargo run --release -- s result-sobel-h.png result-sobel-v.png result-sobel-m.png
cargo run --release -- s result-laplacian_8r.png result-laplacian_8.png result-laplacian_8m.png
