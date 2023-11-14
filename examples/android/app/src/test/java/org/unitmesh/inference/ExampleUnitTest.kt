package org.unitmesh.inference


import org.junit.Assert.*
import org.junit.Test
import org.unitmesh.bindgen.Document
import org.unitmesh.bindgen.Metadata
import org.unitmesh.bindgen.InMemoryEmbeddingStore

/**
 * Example local unit test, which will execute on the development machine (host).
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
class ExampleUnitTest {
    @Test
    fun addition_isCorrect() {
        assertEquals(4, 2 + 2)
        val store = InMemoryEmbeddingStore()
        val metadata = Metadata(mapOf())
        store.add("hello", listOf(0.1f), Document("", metadata, "", listOf()))
        store.findRelevant(listOf(0.1f), 1, 0.5f)
    }
}