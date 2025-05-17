<script lang="ts" setup>
import { onMounted, ref } from 'vue'

interface TodoItem {
  id: string
  label: string
  isEdit: boolean
}

const todoList = ref<TodoItem[]>([])
const setupTodoList = () => {
  const cache = window.localStorage.getItem('todoList')
  if (cache) todoList.value = JSON.parse(cache)
}

const addTextRef = ref('')
const editTextRef = ref('')
const editItem = ref<TodoItem | null>(null)
const isComposing = ref(false)

const handleAddTodo = () => {
  const text = addTextRef.value.trim()
  if (!text) return
  const newItem: TodoItem = {
    id: Date.now().toString(),
    label: text,
    isEdit: false
  }
  window.localStorage.setItem(
    'todoList',
    JSON.stringify([...todoList.value, newItem])
  )
  setupTodoList()
  addTextRef.value = ''
}

const handleDelete = (id: string) => {
  const filtered = todoList.value.filter(i => i.id !== id)
  window.localStorage.setItem('todoList', JSON.stringify(filtered))
  setupTodoList()
}

const onEditClick = (item: TodoItem) => {
  editItem.value = item
  editTextRef.value = item.label
  setTimeout(() => {
    document.getElementById(`edit-input-${item.id}`)?.focus()
  }, 50)
}

const handleEditTodo = () => {
  if (!editItem.value) return
  const updated = todoList.value.map(i =>
    i.id === editItem.value!.id
      ? { ...i, label: editTextRef.value }
      : i
  )
  window.localStorage.setItem('todoList', JSON.stringify(updated))
  setupTodoList()
  editItem.value = null
  editTextRef.value = ''
}

// 組字開始／結束
const onCompositionStart = () => { isComposing.value = true }
const onCompositionEnd = () => { isComposing.value = false }

// 新增輸入框按下 Enter
const onAddKeydown = (e: KeyboardEvent) => {
  // Safari 中文輸入階段 keyCode 會是 229；其他瀏覽器可以用 e.isComposing
  const composing = e.isComposing || e.keyCode === 229 || isComposing.value
  if (e.key === 'Enter' && !composing) {
    e.preventDefault()
    handleAddTodo()
  }
}

// 編輯輸入框按下 Enter
const onEditKeydown = (e: KeyboardEvent) => {
  const composing = e.isComposing || e.keyCode === 229 || isComposing.value
  if (e.key === 'Enter' && !composing) {
    e.preventDefault()
    handleEditTodo()
  }
}

const handleCancelEdit = () => {
  editItem.value = null
  editTextRef.value = ''
}

onMounted(() => {
  setupTodoList()
})
</script>

<template>
  <section class="w-full h-screen bg-#B3B7EE p-2 space-y-4">
    <!-- 新增區 -->
    <div class="sticky top-2 bg-white flex items-center outline outline-2px outline-black rounded-md overflow-hidden">
      <input
        v-model="addTextRef"
        @compositionstart="onCompositionStart"
        @compositionupdate="onCompositionStart"
        @compositionend="onCompositionEnd"
        @keydown="onAddKeydown"
        type="text"
        class="w-full py-2 px-4 placeholder:text-sm focus:outline-none text-sm"
        placeholder="請輸入待辦事項..."
      />
      <button @click="handleAddTodo" class="w-10 h-10 outline outline-black bg-#9395D3 text-white text-xl">
        <img src="./assets/add.svg" alt="add" class="w-5 h-5 stroke-white mx-auto bg-transparent">
      </button>
    </div>

    <!-- 列表區，帶 TransitionGroup -->
    <Transition name="fade" mode="out-in">
      <TransitionGroup v-if="todoList.length > 0" name="fade" tag="ul" class="space-y-2">
        <li
          v-for="item in todoList.slice().reverse()"
          :key="item.id"
          class="flex items-center justify-between rounded-md p-4 shadow-[0px_4px_4px_0px_#00000040] bg-white hover:outline hover:outline-2px hover:outline-black"
        >
          <!-- 顯示 or 編輯 切換 -->
          <div class="flex-1">
            <template v-if="item.id !== editItem?.id">
              <p @dblclick="onEditClick(item)" class="text-sm">{{ item.label }}</p>
            </template>
            <template v-else>
              <input
                v-model="editTextRef"
                @compositionstart="onCompositionStart"
                @compositionupdate="onCompositionStart"
                @compositionend="onCompositionEnd"
                @keydown="onEditKeydown"
                @blur="handleCancelEdit"
                :id="`edit-input-${item.id}`"
                type="text"
                class="w-full placeholder:text-sm bg-stone-100 mr-2 text-sm focus:outline-none"
              />
            </template>
          </div>
          <!-- 按鈕 -->
          <div class="flex items-center space-x-2">
            <button @click="onEditClick(item)" class="px-1 bg-white">
              <img src="./assets/edit.svg" alt="edit" class="w-5 h-5" />
            </button>
            <button @click="handleDelete(item.id)" class="px-1 bg-white">
              <img src="./assets/delete.svg" alt="delete" class="w-5 h-5" />
            </button>
          </div>
        </li>
      </TransitionGroup>
      <p v-else class="text-center text-xl text-black/80 font-bold pt-20 fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 whitespace-nowrap pb-20">
        Nice! 目前沒有待辦事項～
      </p>
    </Transition>
  </section>
</template>

<style>
*::-webkit-scrollbar { width: 7px; }
*::-webkit-scrollbar-thumb { border-radius: 4px; background-color: #b7ff4a; border: 1px solid slategrey; }

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.4s;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
.fade-enter-to,
.fade-leave-from {
  opacity: 1;
}
</style>
