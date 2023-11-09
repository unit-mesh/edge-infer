## API Design

```

```

```koltin
interface EmbeddingStore<Embedded> {
    /**
     * Adds an embedding to the store and returns its unique identifier.
     *
     * @param embedding The embedding to be added.
     * @return A unique identifier associated with the added embedding.
     */
    fun add(embedding: Embedding): String

    /**
     * Adds an embedding to the store with a specified identifier.
     *
     * @param id The unique identifier for the embedding.
     * @param embedding The embedding to be added.
     */
    fun add(id: String, embedding: Embedding)

    /**
     * Adds an embedding to the store and associates it with the provided embedded data.
     *
     * @param embedding The embedding to be added.
     * @param embedded The data embedded in the store.
     * @return A unique identifier associated with the added embedding.
     */
    fun add(embedding: Embedding, embedded: Embedded): String

    /**
     * Adds a list of embeddings to the store and returns a list of unique identifiers.
     *
     * @param embeddings The list of embeddings to be added.
     * @return A list of unique identifiers associated with the added embeddings, in the same order.
     */
    fun addAll(embeddings: List<Embedding>): List<String>

    /**
     * Adds a list of embeddings to the store and associates them with a list of embedded data.
     *
     * @param embeddings The list of embeddings to be added.
     * @param embedded The list of data embedded in the store.
     * @return A list of unique identifiers associated with the added embeddings, in the same order.
     */
    fun addAll(embeddings: List<Embedding>, embedded: List<Embedded>): List<String>

    /**
     * Find relevant embeddings in the store based on a reference embedding, with a maximum number of results.
     * An optional minimum score can be specified to filter results.
     *
     * @param referenceEmbedding The reference embedding to compare against.
     * @param maxResults The maximum number of results to retrieve.
     * @param minScore The minimum similarity score required to include a result (default is 0.0).
     * @return A list of [EmbeddingMatch] objects representing relevant matches.
     */
    fun findRelevant(
        referenceEmbedding: Embedding,
        maxResults: Int,
        minScore: Double = 0.0,
    ): List<EmbeddingMatch<Embedded>>
}
```

