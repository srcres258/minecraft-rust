pub struct RenderMaster {
    // Chunks
    chunk_renderer: ChunkRenderer,
    water_renderer: WaterRenderer,
    flora_renderer: FloraRenderer,

    // Detail
    skybox_renderer: SkyboxRenderer,
    
    draw_box: bool
}