# Convolver

Convolver 是一个通用的图像卷积命令行工具。基于 Rayon 并行加速，充分利用多核 CPU。

## 安装

首先安装 Rust 和 Cargo。

```sh
git clone https://github.com/KaiHuaDou/convolver.git --depth 1
cd convolver
cargo run --release
```

## 使用方法

> [!NOTE]
> 输入和输出必须是 RGBA 模式的 PNG，可以使用 `python convert.py` 进行转换。

### 图像卷积

```sh
convolver c <输入图片> <输出图片> -f <卷积方法> -t <迭代次数> -i <进度指示器>
```

### 支持的卷积方法

> [!NOTE]
> 以下 `*: usize` 均指卷积核大小
> 以下 `d: String` 均指方向，可选 `n`,`ne`,`e`,`se`,`s`,`sw`,`w`,`nw`。

- `*-blur`：均值模糊
- `*-dog-σ1-σ2`：高斯差分 (DoG)，`σ1: f32`, `σ2: f32` 均为标准差
- `*-emboss-d`：浮雕效果
- `*-gauss-blur-σ`：高斯模糊，`σ: f32` 为标准差
- `*-gauss-sharpen-σ`：高斯锐化，`σ: f32` 为标准差
- `*-motion-l-θ`：运动模糊，`l: f32` 为长度，`θ: f32` 为角度
- `*-none`：无操作
- `3-edge`：边缘增强
- `3-kirsch-d`：Kirsch 边缘检测
- `3-laplacian_4` / `3-laplacian_8` / `3-laplacian_8r`：拉普拉斯算子
- `3-prewitt_h` / `3-prewitt_v`：Prewitt 边缘检测
- `3-robinson-d`：Robinson 边缘检测
- `3-scharr_h` / `3-scharr_v`：Scharr 边缘检测
- `3-sharpen` / `3-sharpen+`：一般锐化效果
- `3-sobel_h` / `3-sobel_v`：Sobel 边缘检测
- `3-unsharp_masking`：Unsharp masking 效果
- `5-laplacian_og`：高斯拉普拉斯

### 图像合并

```sh
convolver s <图片1> <图片2> <输出图片>
```

## 许可证

本项目采用 [GNU GPLv3](LICENSE) 协议开源。欢迎提交 Issue 或 PR！
