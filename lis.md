Example	ST reference	What it does
f469disco-slideshow	LCD_PicturesFromSDCard	Two layers, crossfade by varying layer transparency; cycles ColorBars, SolidRed, SolidGreen, SolidBlue, Gradient.
f469disco-animated-layers	LCD_AnimatedPictureFromSDCard	Gradient on L1, sprites on L2 with color key (0x0000); cycles red/green/blue/white-circle every 800 ms.
f469disco-paint	LCD_Paint	Touch drawing: 8-color palette at top, brush (16×16) below; raw buffer writes for drawing (no SD/BMP). Touch INT uses PC1 (PC0 is FMC_SDNWE when SDRAM is used).
f469disco-image-slider	LCD_DSI_ImagesSlider (simplified)	Swipe left/right to change image; double buffer + set_layer_buffer_address to flip between patterns (no DSI command mode / TE).
Build commands (release, recommended for flash size):
