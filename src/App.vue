<script lang="ts" setup>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const DEFAULT_CATEGORY = '未分類'

interface CopyList {
  id: string
  title: string
  label: string
  category: string
  isEdit: boolean
}

// 深色模式
const isDark = ref(false)

const setupDarkMode = () => {
  const cache = window.localStorage.getItem('darkMode')
  isDark.value = cache === 'true'
  applyDarkMode()
}

const toggleDarkMode = () => {
  isDark.value = !isDark.value
  window.localStorage.setItem('darkMode', String(isDark.value))
  applyDarkMode()
}

const applyDarkMode = () => {
  document.documentElement.classList.toggle('dark', isDark.value)
}

// 拖曳排序
const dragIndex = ref<number | null>(null)
const dragOverIndex = ref<number | null>(null)

const onDragStart = (index: number) => {
  dragIndex.value = index
}

const onDragOver = (e: DragEvent, index: number) => {
  e.preventDefault()
  dragOverIndex.value = index
}

const onDragEnd = () => {
  if (dragIndex.value !== null && dragOverIndex.value !== null && dragIndex.value !== dragOverIndex.value) {
    // 拖曳的是 filteredCopyList 中的索引，需要對應回 copyList 的真實索引
    const filtered = filteredCopyList.value
    const dragItem = filtered[dragIndex.value]
    const targetItem = filtered[dragOverIndex.value]
    const realFrom = copyList.value.findIndex(i => i.id === dragItem.id)
    const realTo = copyList.value.findIndex(i => i.id === targetItem.id)
    if (realFrom !== -1 && realTo !== -1) {
      const [moved] = copyList.value.splice(realFrom, 1)
      copyList.value.splice(realTo, 0, moved)
      window.localStorage.setItem('copyList', JSON.stringify(copyList.value))
    }
  }
  dragIndex.value = null
  dragOverIndex.value = null
}

const isAdding = ref(false)

// 類別管理
const categories = ref<string[]>([])
const activeCategory = ref<string | null>(null)
const isAddingCategory = ref(false)
const newCategoryName = ref('')
const categoryInput = ref<HTMLInputElement | null>(null)
const isComposing = ref(false)
const editingCategory = ref<string | null>(null)
const editCategoryName = ref('')

const setupCategories = () => {
  const cache = window.localStorage.getItem('categories')
  categories.value = cache ? JSON.parse(cache) : []
}

const saveCategories = () => {
  window.localStorage.setItem('categories', JSON.stringify(categories.value))
}

const inlineNewCategory = ref('')
const isInlineAddingCategory = ref(false)

const handleAddCategory = () => {
  const name = newCategoryName.value.trim()
  if (!name || categories.value.includes(name)) return
  categories.value.push(name)
  saveCategories()
  newCategoryName.value = ''
  isAddingCategory.value = false
}

const handleInlineAddCategory = (selectId: string) => {
  const name = inlineNewCategory.value.trim()
  if (!name || categories.value.includes(name)) return
  categories.value.push(name)
  saveCategories()
  inlineNewCategory.value = ''
  isInlineAddingCategory.value = false
  selectOption(selectId, name)
}

const deleteCategoryConfirm = ref<string | null>(null)

const requestDeleteCategory = (cat: string) => {
  deleteCategoryConfirm.value = cat
}

const confirmDeleteCategory = () => {
  const cat = deleteCategoryConfirm.value
  if (!cat) return
  categories.value = categories.value.filter(c => c !== cat)
  saveCategories()
  // 將該類別的項目歸回未分類
  copyList.value.forEach(item => {
    if (item.category === cat) item.category = DEFAULT_CATEGORY
  })
  window.localStorage.setItem('copyList', JSON.stringify(copyList.value))
  if (activeCategory.value === cat) activeCategory.value = null
  // 如果當前選中的類別被刪除，回到未分類
  if (selectedCategory.value === cat) selectedCategory.value = DEFAULT_CATEGORY
  if (editCategory.value === cat) editCategory.value = DEFAULT_CATEGORY
  deleteCategoryConfirm.value = null
}

const cancelDeleteCategory = () => {
  deleteCategoryConfirm.value = null
}

const startEditCategory = (cat: string) => {
  editingCategory.value = cat
  editCategoryName.value = cat
}

const handleEditCategory = () => {
  const newName = editCategoryName.value.trim()
  if (!newName || (newName !== editingCategory.value && categories.value.includes(newName))) return
  const oldName = editingCategory.value!
  const idx = categories.value.indexOf(oldName)
  if (idx !== -1) categories.value[idx] = newName
  saveCategories()
  // 更新所有項目的類別名稱
  copyList.value.forEach(item => {
    if (item.category === oldName) item.category = newName
  })
  window.localStorage.setItem('copyList', JSON.stringify(copyList.value))
  if (activeCategory.value === oldName) activeCategory.value = newName
  editingCategory.value = null
}

// 資料管理
const copyList = ref<CopyList[]>([])
const setupCopyList = () => {
  const copyCache = window.localStorage.getItem('copyList')

  if (!copyCache) return

  // 相容舊資料：補上 category 欄位
  const parsed: CopyList[] = JSON.parse(copyCache)
  let migrated = false
  parsed.forEach(item => {
    if (!item.category) {
      item.category = DEFAULT_CATEGORY
      migrated = true
    }
  })
  if (migrated) {
    window.localStorage.setItem('copyList', JSON.stringify(parsed))
  }

  copyList.value = parsed
  isAdding.value = false
}

const hasUncategorized = computed(() => copyList.value.some(item => item.category === DEFAULT_CATEGORY))

const filteredCopyList = computed(() => {
  if (!activeCategory.value) return copyList.value
  return copyList.value.filter(item => item.category === activeCategory.value)
})

const copyTitle = ref('')
const textModel = ref('')
const selectedCategory = ref(DEFAULT_CATEGORY)

const handleAddCopy = () => {
  const newText = textModel.value.trim()

  if (!newText || !copyTitle.value) return

  const payload: CopyList = {
    id: new Date().getTime().toString(),
    title: copyTitle.value,
    label: newText,
    category: selectedCategory.value,
    isEdit: false
  }
  window.localStorage.setItem('copyList', JSON.stringify([...copyList.value, payload]))
  setupCopyList()

  copyTitle.value = ''
  textModel.value = ''
  selectedCategory.value = DEFAULT_CATEGORY
}

const editId = ref('')

// 編輯項目
const editingItemId = ref<string | null>(null)
const editTitle = ref('')
const editLabel = ref('')
const editCategory = ref(DEFAULT_CATEGORY)

const startEditItem = (item: CopyList) => {
  editingItemId.value = item.id
  editTitle.value = item.title
  editLabel.value = item.label
  editCategory.value = item.category
  editId.value = ''
}

const cancelEditItem = () => {
  editingItemId.value = null
}

const handleSaveEdit = () => {
  if (!editTitle.value.trim() || !editLabel.value.trim()) return
  const item = copyList.value.find(i => i.id === editingItemId.value)
  if (!item) return
  item.title = editTitle.value.trim()
  item.label = editLabel.value.trim()
  item.category = editCategory.value
  window.localStorage.setItem('copyList', JSON.stringify(copyList.value))
  editingItemId.value = null
  if (activeCategory.value === DEFAULT_CATEGORY && !hasUncategorized.value) {
    activeCategory.value = null
  }
}

// 刪除確認
const deleteConfirmId = ref<string | null>(null)

const requestDelete = (id: string) => {
  deleteConfirmId.value = id
  editId.value = ''
}

const confirmDelete = () => {
  if (!deleteConfirmId.value) return
  const newCopyList = copyList.value.filter(item => item.id !== deleteConfirmId.value)
  window.localStorage.setItem('copyList', JSON.stringify(newCopyList))
  deleteConfirmId.value = null
  setupCopyList()
  if (activeCategory.value === DEFAULT_CATEGORY && !hasUncategorized.value) {
    activeCategory.value = null
  }
}

const cancelDelete = () => {
  deleteConfirmId.value = null
}

// 呼叫 Rust 關閉視窗
const handleHideWindow = () => {
  invoke('tauri', { cmd: 'hide_window' })
}

const handleCopy = (text: string) => {
  navigator.clipboard.writeText(text)
  handleHideWindow()
}

const onAddClick = () => {
  isAdding.value = true
  setTimeout(() => {
    const title = document.getElementById('add__title')
    title?.focus()
  }, 50);
}

// 自訂下拉選單
const openDropdown = ref<string | null>(null)
const dropdownRefs = ref<Record<string, HTMLElement | null>>({})

const toggleDropdown = (id: string) => {
  openDropdown.value = openDropdown.value === id ? null : id
  isInlineAddingCategory.value = false
  inlineNewCategory.value = ''
}

const selectOption = (id: string, value: string) => {
  if (id === 'add') selectedCategory.value = value
  else if (id === 'edit') editCategory.value = value
  openDropdown.value = null
}

const getCategoryOptions = computed(() => [DEFAULT_CATEGORY, ...categories.value])

const handleClickOutside = (e: MouseEvent) => {
  if (!openDropdown.value) return
  const el = dropdownRefs.value[openDropdown.value]
  if (el && !el.contains(e.target as Node)) {
    openDropdown.value = null
    isInlineAddingCategory.value = false
    inlineNewCategory.value = ''
  }
}

onMounted(() => {
  setupDarkMode()
  setupCategories()
  setupCopyList()
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<template>
  <section class="h-100dvh flex flex-col section-bg">
    <!-- 頂部列（可拖曳視窗） -->
    <div data-tauri-drag-region class="flex items-center justify-between px-3 pt-2.5 pb-0 shrink-0 cursor-move">
      <span data-tauri-drag-region class="text-[11px] fw-600 text-stone-400 dark:text-stone-500 tracking-wide">QUICK COPY</span>
      <button @click="toggleDarkMode" class="w-6 h-6 flex items-center justify-center rounded-full hover:bg-stone-200/60 dark:hover:bg-stone-700/60 transition-colors">
        <!-- 太陽 / 月亮 -->
        <svg v-if="isDark" class="w-3.5 h-3.5 text-amber-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
        </svg>
        <svg v-else class="w-3.5 h-3.5 text-stone-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 12.79A9 9 0 1111.21 3 7 7 0 0021 12.79z"/>
        </svg>
      </button>
    </div>
    <!-- 類別標籤列 -->
    <div class="flex flex-nowrap items-center gap-1.5 px-3 pt-1.5 pb-2 overflow-x-auto category-tabs shrink-0">
      <button
        @click="activeCategory = null"
        class="tab-btn"
        :class="activeCategory === null ? 'tab-active' : 'tab-inactive'"
      >全部</button>
      <button
        v-if="hasUncategorized"
        @click="activeCategory = DEFAULT_CATEGORY"
        class="tab-btn"
        :class="activeCategory === DEFAULT_CATEGORY ? 'tab-active' : 'tab-inactive'"
      >{{ DEFAULT_CATEGORY }}</button>
      <template v-for="cat in categories" :key="cat">
        <div v-if="editingCategory === cat" class="shrink-0 flex items-center gap-1">
          <input
            v-model="editCategoryName"
            @compositionstart="isComposing = true"
            @compositionend="isComposing = false"
            @keydown.enter="!isComposing && handleEditCategory()"
            @keydown.esc="editingCategory = null"
            class="w-16 px-2 py-0.5 text-xs border border-[#c8e66a] rounded-full focus:outline-none bg-white"
            autofocus
          />
          <button @click="handleEditCategory" class="w-5.5 h-5.5 flex items-center justify-center rounded-full bg-[#e8f8c8] text-[#7ab32a] text-xs fw-700 hover:bg-[#d4f0a0] transition-colors">✓</button>
          <button @click="editingCategory = null" class="w-5.5 h-5.5 flex items-center justify-center rounded-full bg-stone-100 text-stone-400 text-xs hover:bg-stone-200 transition-colors">✕</button>
        </div>
        <button
          v-else
          @click="activeCategory = cat"
          @dblclick.prevent="startEditCategory(cat)"
          class="group tab-btn flex items-center gap-0.5"
          :class="activeCategory === cat ? 'tab-active' : 'tab-inactive'"
        >
          {{ cat }}
          <span
            @click.stop="requestDeleteCategory(cat)"
            class="inline-flex items-center justify-center w-3.5 h-3.5 rounded-full text-[10px] leading-none opacity-50 hover:opacity-100 hover:bg-black/10"
          >✕</span>
        </button>
      </template>
      <!-- 新增類別按鈕 -->
      <button
        @click="isAddingCategory = true"
        class="shrink-0 w-5.5 h-5.5 text-[11px] rounded-full border border-dashed border-[var(--text-muted)] text-balck hover:border-[#a0d911] hover:text-[#7ab32a] flex items-center justify-center transition-colors"
      >+</button>
    </div>

    <!-- 項目列表 -->
    <TransitionGroup tag="ul" name="list" class="flex-1 overflow-auto space-y-2 px-3 pb-2">
      <li
        v-for="(item, index) in filteredCopyList"
        :key="item.id"
        class="card group relative"
        :class="{ 'drag-over': dragOverIndex === index && dragIndex !== index }"
        draggable="true"
        @dragstart="onDragStart(index)"
        @dragover="onDragOver($event, index)"
        @dragend="onDragEnd"
      >
        <!-- 編輯模式 -->
        <template v-if="editingItemId === item.id">
          <div class="space-y-2">
            <input
              v-model="editTitle"
              placeholder="標題"
              class="input-field fw-600"
              @keydown.esc="cancelEditItem"
              autofocus
            >
            <div class="custom-select" :ref="(el) => dropdownRefs['edit'] = el as HTMLElement">
              <button type="button" class="select-trigger" @click="toggleDropdown('edit')">
                <span>{{ editCategory }}</span>
                <svg class="select-arrow" :class="{ 'rotate-180': openDropdown === 'edit' }" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"/></svg>
              </button>
              <Transition name="dropdown">
                <ul v-if="openDropdown === 'edit'" class="select-options">
                  <li
                    v-for="opt in getCategoryOptions"
                    :key="opt"
                    @click="selectOption('edit', opt)"
                    class="select-option group/opt flex items-center justify-between"
                    :class="{ 'select-option-active': editCategory === opt }"
                  >
                    <span>{{ opt }}</span>
                    <span
                      v-if="opt !== DEFAULT_CATEGORY"
                      @click.stop="requestDeleteCategory(opt)"
                      class="hidden group-hover/opt:flex w-4 h-4 items-center justify-center rounded-full text-[9px] text-stone-400 hover:text-red-500 hover:bg-red-50 transition-colors"
                    >✕</span>
                  </li>
                  <!-- 快速新增類別 -->
                  <li class="border-t border-[var(--border-light)] mt-1 pt-1">
                    <div v-if="isInlineAddingCategory" class="flex items-center gap-1 px-1">
                      <input
                        v-model="inlineNewCategory"
                        @compositionstart="isComposing = true"
                        @compositionend="isComposing = false"
                        @keydown.enter="!isComposing && handleInlineAddCategory('edit')"
                        @keydown.esc="isInlineAddingCategory = false, inlineNewCategory = ''"
                        placeholder="類別名稱"
                        class="flex-1 min-w-0 px-2 py-1 text-[11px] border border-[#c8e66a] rounded-md focus:outline-none bg-white"
                        @click.stop
                        autofocus
                      />
                      <button @click.stop="handleInlineAddCategory('edit')" class="w-5.5 h-5.5 shrink-0 flex items-center justify-center rounded-md bg-[#e8f8c8] text-[#7ab32a] text-[10px] fw-700 hover:bg-[#d4f0a0] transition-colors">✓</button>
                    </div>
                    <button
                      v-else
                      @click.stop="isInlineAddingCategory = true"
                      class="w-full flex items-center gap-1.5 px-2 py-1.5 text-[11px] text-stone-400 hover:text-[#7ab32a] rounded-md hover:bg-[#f8fef0] transition-colors"
                    >
                      <span class="text-[13px] leading-none">+</span> 新增類別
                    </button>
                  </li>
                </ul>
              </Transition>
            </div>
            <textarea
              v-model="editLabel"
              placeholder="輸入要複製的文字內容..."
              rows="3"
              class="input-field resize-none"
              @keydown.esc="cancelEditItem"
            />
            <div class="flex gap-2">
              <button @click="cancelEditItem" class="btn-secondary flex-1 !py-1.5 !text-[11px]">取消</button>
              <button @click="handleSaveEdit" class="btn-primary flex-1 !py-1.5 !text-[11px]">儲存</button>
            </div>
          </div>
        </template>

        <!-- 顯示模式 -->
        <template v-else>
          <div
            class="relative z-1"
            @mouseover="editId = item.id"
            @mouseleave="editId = ''"
          >
            <div class="flex items-center gap-1.5 mb-0.5">
              <p class="line-clamp-1 text-[13px] fw-700 text-[var(--text-primary)]">{{ item.title }}</p>
              <span class="shrink-0 text-[9px] leading-none px-1.5 py-[3px] rounded-full bg-[var(--tag-bg)] text-[var(--tag-text)] fw-500">{{ item.category }}</span>
            </div>
            <p class="text-[11px] text-[var(--text-faint)] whitespace-pre-line line-clamp-2 leading-relaxed">{{ item.label }}</p>

            <!-- Hover 操作覆蓋層 -->
            <Transition name="fade" mode="in-out">
              <div v-if="editId === item.id" class="absolute inset-0 z-2 flex rounded-lg overflow-hidden">
                <button @click="handleCopy(item.label)" class="flex-1 flex items-center justify-center gap-1 bg-[#b7ff4a]/20 hover:bg-[#b7ff4a]/40 backdrop-blur-sm transition-colors">
                  <img src="./assets/copy.svg" alt="Copy" class="w-3.5 h-3.5 icon-adapt">
                  <span class="text-[10px] fw-600 text-[var(--text-muted)]">複製</span>
                </button>
                <div class="w-px bg-white/60"></div>
                <button @click="startEditItem(item)" class="flex-1 flex items-center justify-center gap-1 bg-blue-500/10 hover:bg-blue-500/20 backdrop-blur-sm transition-colors">
                  <svg class="w-3.5 h-3.5 icon-adapt" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>
                    <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
                  </svg>
                  <span class="text-[10px] fw-600 text-[var(--text-muted)]">編輯</span>
                </button>
                <div class="w-px bg-white/60"></div>
                <button @click="requestDelete(item.id)" class="flex-1 flex items-center justify-center gap-1 bg-red-500/10 hover:bg-red-500/20 backdrop-blur-sm transition-colors">
                  <img src="./assets/delete.svg" alt="Delete" class="w-3.5 h-3.5 icon-adapt">
                  <span class="text-[10px] fw-600 text-[var(--text-muted)]">刪除</span>
                </button>
              </div>
            </Transition>
          </div>
        </template>
      </li>

      <!-- 空狀態 -->
      <li v-if="filteredCopyList.length === 0 && !isAdding" key="empty" class="flex flex-col items-center justify-center pt-20 text-[var(--text-faint)]">
        <svg class="w-10 h-10 mb-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
          <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
        </svg>
        <p class="text-sm fw-600">{{ activeCategory ? '此類別沒有項目' : '尚無快速複製項目' }}</p>
        <p class="text-xs mt-1">點擊下方按鈕開始新增</p>
      </li>

    </TransitionGroup>

    <!-- 底部按鈕列 -->
    <div class="shrink-0 px-3 pb-3 pt-2 border-t border-[var(--border-light)] dark:border-stone-800 section-bg">
      <button @click="onAddClick" class="btn-primary w-full">
        <span class="mr-1">+</span> 新增項目
      </button>
    </div>

    <!-- 新增項目抽屜 -->
    <Transition name="drawer-overlay">
      <div v-if="isAdding" class="fixed inset-0 z-50 bg-black/30 backdrop-blur-[2px]" @click.self="isAdding = false, textModel = '', copyTitle = '', selectedCategory = DEFAULT_CATEGORY"></div>
    </Transition>
    <Transition name="drawer">
      <div v-if="isAdding" class="fixed bottom-0 left-0 right-0 z-50 drawer-panel rounded-t-2xl shadow-lg p-4 space-y-3">
        <div class="w-8 h-1 rounded-full bg-[var(--border)] mx-auto"></div>
        <p class="text-[13px] fw-700 text-[var(--text-primary)]">新增項目</p>
        <input
          id="add__title"
          type="text"
          v-model="copyTitle"
          placeholder="標題"
          class="input-field fw-600"
        >
        <div class="custom-select" :ref="(el) => dropdownRefs['add'] = el as HTMLElement">
          <button type="button" class="select-trigger" @click="toggleDropdown('add')">
            <span>{{ selectedCategory }}</span>
            <svg class="select-arrow" :class="{ 'rotate-180': openDropdown === 'add' }" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"/></svg>
          </button>
          <Transition name="dropdown">
            <ul v-if="openDropdown === 'add'" class="select-options">
              <li
                v-for="opt in getCategoryOptions"
                :key="opt"
                @click="selectOption('add', opt)"
                class="select-option group/opt flex items-center justify-between"
                :class="{ 'select-option-active': selectedCategory === opt }"
              >
                <span>{{ opt }}</span>
                <span
                  v-if="opt !== DEFAULT_CATEGORY"
                  @click.stop="requestDeleteCategory(opt)"
                  class="hidden group-hover/opt:flex w-4 h-4 items-center justify-center rounded-full text-[9px] text-stone-400 hover:text-red-500 hover:bg-red-50 transition-colors"
                >✕</span>
              </li>
              <!-- 快速新增類別 -->
              <li class="border-t border-[var(--border-light)] mt-1 pt-1">
                <div v-if="isInlineAddingCategory" class="flex items-center gap-1 px-1">
                  <input
                    v-model="inlineNewCategory"
                    @compositionstart="isComposing = true"
                    @compositionend="isComposing = false"
                    @keydown.enter="!isComposing && handleInlineAddCategory('add')"
                    @keydown.esc="isInlineAddingCategory = false, inlineNewCategory = ''"
                    placeholder="類別名稱"
                    class="flex-1 min-w-0 px-2 py-1 text-[11px] border border-[#c8e66a] rounded-md focus:outline-none bg-white"
                    @click.stop
                    autofocus
                  />
                  <button @click.stop="handleInlineAddCategory('add')" class="w-5.5 h-5.5 shrink-0 flex items-center justify-center rounded-md bg-[#e8f8c8] text-[#7ab32a] text-[10px] fw-700 hover:bg-[#d4f0a0] transition-colors">✓</button>
                </div>
                <button
                  v-else
                  @click.stop="isInlineAddingCategory = true"
                  class="w-full flex items-center gap-1.5 px-2 py-1.5 text-[11px] text-stone-400 hover:text-[#7ab32a] rounded-md hover:bg-[#f8fef0] transition-colors"
                >
                  <span class="text-[13px] leading-none">+</span> 新增類別
                </button>
              </li>
            </ul>
          </Transition>
        </div>
        <textarea
          v-model="textModel"
          placeholder="輸入要複製的文字內容..."
          rows="3"
          class="input-field resize-none"
        />
        <div class="flex gap-2">
          <button @click="isAdding = false, textModel = '', copyTitle = '', selectedCategory = DEFAULT_CATEGORY" class="btn-secondary flex-1">取消</button>
          <button @click="handleAddCopy" class="btn-primary flex-1">新增</button>
        </div>
      </div>
    </Transition>
    <!-- 新增類別抽屜 -->
    <Transition name="drawer-overlay">
      <div v-if="isAddingCategory" class="fixed inset-0 z-50 bg-black/30 backdrop-blur-[2px]" @click.self="isAddingCategory = false, newCategoryName = ''"></div>
    </Transition>
    <Transition name="drawer" @after-enter="categoryInput?.focus()">
      <div v-if="isAddingCategory" class="fixed bottom-0 left-0 right-0 z-50 drawer-panel rounded-t-2xl shadow-lg p-4 space-y-3">
        <div class="w-8 h-1 rounded-full bg-[var(--border)] mx-auto"></div>
        <p class="text-[13px] fw-700 text-[var(--text-primary)]">新增類別</p>
        <input
          ref="categoryInput"
          v-model="newCategoryName"
          @compositionstart="isComposing = true"
          @compositionend="isComposing = false"
          @keydown.enter="!isComposing && handleAddCategory()"
          @keydown.esc="isAddingCategory = false, newCategoryName = ''"
          placeholder="輸入類別名稱"
          class="input-field"
        />
        <div class="flex gap-2">
          <button @click="isAddingCategory = false, newCategoryName = ''" class="btn-secondary flex-1">取消</button>
          <button @click="handleAddCategory" class="btn-primary flex-1">新增</button>
        </div>
      </div>
    </Transition>

    <!-- 刪除項目確認抽屜 -->
    <Transition name="drawer-overlay">
      <div v-if="deleteConfirmId" class="fixed inset-0 z-[100] bg-black/30 backdrop-blur-[2px]" @click.self="cancelDelete"></div>
    </Transition>
    <Transition name="drawer">
      <div v-if="deleteConfirmId" class="fixed bottom-0 left-0 right-0 z-[100] drawer-panel rounded-t-2xl shadow-lg p-4 space-y-3">
        <div class="w-8 h-1 rounded-full bg-[var(--border)] mx-auto"></div>
        <p class="text-[13px] fw-700 text-[var(--text-primary)]">確定要刪除嗎？</p>
        <p class="text-[11px] text-[var(--text-faint)] leading-relaxed">刪除後將無法復原</p>
        <div class="flex gap-2">
          <button @click="cancelDelete" class="btn-secondary flex-1">取消</button>
          <button @click="confirmDelete" class="flex-1 py-2 text-[13px] fw-600 text-white bg-red-500 rounded-lg hover:bg-red-600 transition-colors">刪除</button>
        </div>
      </div>
    </Transition>
    <!-- 刪除類別確認抽屜 -->
    <Transition name="drawer-overlay">
      <div v-if="deleteCategoryConfirm" class="fixed inset-0 z-[100] bg-black/30 backdrop-blur-[2px]" @click.self="cancelDeleteCategory"></div>
    </Transition>
    <Transition name="drawer">
      <div v-if="deleteCategoryConfirm" class="fixed bottom-0 left-0 right-0 z-[100] drawer-panel rounded-t-2xl shadow-lg p-4 space-y-3">
        <div class="w-8 h-1 rounded-full bg-[var(--border)] mx-auto"></div>
        <p class="text-[13px] fw-700 text-[var(--text-primary)]">確定要刪除類別嗎？</p>
        <p class="text-[11px] text-[var(--text-faint)] leading-relaxed">「{{ deleteCategoryConfirm }}」下的項目將歸為未分類</p>
        <div class="flex gap-2">
          <button @click="cancelDeleteCategory" class="btn-secondary flex-1">取消</button>
          <button @click="confirmDeleteCategory" class="flex-1 py-2 text-[13px] fw-600 text-white bg-red-500 rounded-lg hover:bg-red-600 transition-colors">刪除</button>
        </div>
      </div>
    </Transition>
  </section>
</template>

<style>
/* === CSS 變數 === */
:root {
  --bg: #fafaf9;
  --bg-card: white;
  --bg-input: #fafaf9;
  --bg-drawer: white;
  --border: #e7e5e4;
  --border-light: #f0efed;
  --text-primary: #1c1917;
  --text-secondary: #44403c;
  --text-muted: #78716c;
  --text-faint: #a8a29e;
  --accent: #b7ff4a;
  --accent-hover: #a8f035;
  --accent-bg: #f0fad2;
  --accent-text: #6a9a1f;
  --tag-bg: #f0fad2;
  --tag-text: #6a9a1f;
  --hover-bg: #f5f5f4;
  --scrollbar: #d4e7a5;
  --shadow-card: 0 1px 2px rgba(0,0,0,0.04);
  --shadow-card-hover: 0 2px 8px rgba(0,0,0,0.06);
}

html.dark {
  --bg: #1a1a1a;
  --bg-card: #262626;
  --bg-input: #2a2a2a;
  --bg-drawer: #262626;
  --border: #3a3a3a;
  --border-light: #333;
  --text-primary: #f5f5f4;
  --text-secondary: #d6d3d1;
  --text-muted: #a8a29e;
  --text-faint: #6b6560;
  --accent: #b7ff4a;
  --accent-hover: #a8f035;
  --accent-bg: #2a3a1a;
  --accent-text: #b7ff4a;
  --tag-bg: #2a3a1a;
  --tag-text: #b7ff4a;
  --hover-bg: #333;
  --scrollbar: #4a5a2a;
  --shadow-card: 0 1px 2px rgba(0,0,0,0.2);
  --shadow-card-hover: 0 2px 8px rgba(0,0,0,0.3);
}

/* === 基底 === */
.section-bg { background: var(--bg); }

/* Scrollbar */
*::-webkit-scrollbar { width: 5px; }
*::-webkit-scrollbar-button { background: transparent; }
*::-webkit-scrollbar-track-piece { background: transparent; }
*::-webkit-scrollbar-thumb { border-radius: 99px; background-color: var(--scrollbar); }
*::-webkit-scrollbar-thumb:hover { background-color: var(--accent); }
*::-webkit-scrollbar-track { background: transparent; }

.category-tabs::-webkit-scrollbar { display: none; }
.category-tabs { -ms-overflow-style: none; scrollbar-width: none; }

/* Tab */
.tab-btn {
  flex-shrink: 0; padding: 3px 10px; font-size: 11px;
  font-weight: 500; border-radius: 99px; transition: all 0.2s ease; white-space: nowrap;
}
.tab-active {
  background: var(--text-primary); color: var(--bg);
  border: 1px solid var(--text-primary);
  box-shadow: 0 1px 3px rgba(0,0,0,0.15);
}
.tab-inactive {
  background: var(--bg-card); color: var(--text-muted); border: 1px solid var(--border);
}
.tab-inactive:hover { border-color: var(--text-faint); color: var(--text-secondary); }

/* Card */
.card {
  padding: 10px 14px; background: var(--bg-card); border-radius: 10px;
  border: 1px solid var(--border-light); box-shadow: var(--shadow-card);
  transition: box-shadow 0.2s ease, border-color 0.2s ease, transform 0.15s ease;
}
.card:hover { border-color: var(--border); box-shadow: var(--shadow-card-hover); }

/* Drag */
.card[draggable="true"] { cursor: grab; }
.card[draggable="true"]:active { cursor: grabbing; }
.drag-over { border-color: var(--accent) !important; box-shadow: 0 0 0 2px rgba(183,255,74,0.25) !important; }

/* Input */
.input-field {
  width: 100%; padding: 7px 10px; font-size: 12px;
  color: var(--text-secondary); background: var(--bg-input);
  border: 1px solid var(--border); border-radius: 8px; outline: none;
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}
.input-field::placeholder { color: var(--text-faint); }
.input-field:focus { border-color: var(--accent); box-shadow: 0 0 0 2px rgba(183,255,74,0.2); }

/* Buttons */
.btn-primary {
  padding: 8px 0; font-size: 13px; font-weight: 600;
  color: #1c1917; background: var(--accent); border-radius: 10px;
  transition: all 0.2s ease; box-shadow: 0 1px 3px rgba(183,255,74,0.3);
}
.btn-primary:hover { background: var(--accent-hover); box-shadow: 0 2px 8px rgba(183,255,74,0.4); transform: translateY(-0.5px); }
.btn-primary:active { transform: translateY(0); box-shadow: 0 1px 2px rgba(183,255,74,0.3); }
.btn-secondary {
  padding: 8px 0; font-size: 13px; font-weight: 500;
  color: var(--text-muted); background: var(--bg-card);
  border: 1px solid var(--border); border-radius: 10px; transition: all 0.2s ease;
}
.btn-secondary:hover { border-color: var(--text-faint); color: var(--text-secondary); }

/* Select */
.custom-select { position: relative; }
.select-trigger {
  width: 100%; display: flex; align-items: center; justify-content: space-between;
  padding: 7px 10px; font-size: 12px; color: var(--text-secondary);
  background: var(--bg-input); border: 1px solid var(--border); border-radius: 8px;
  cursor: pointer; transition: border-color 0.2s ease, box-shadow 0.2s ease;
}
.select-trigger:hover { border-color: var(--text-faint); }
.select-trigger:focus { border-color: var(--accent); box-shadow: 0 0 0 2px rgba(183,255,74,0.2); outline: none; }
.select-arrow { transition: transform 0.2s ease; color: var(--text-faint); flex-shrink: 0; }
.select-options {
  position: absolute; z-index: 50; left: 0; right: 0; top: calc(100% + 4px);
  background: var(--bg-card); border: 1px solid var(--border); border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.08); padding: 4px;
  max-height: 130px; overflow-y: auto; list-style: none; margin: 0;
}
.select-option {
  padding: 6px 8px; font-size: 12px; color: var(--text-muted);
  border-radius: 6px; cursor: pointer; transition: background 0.15s ease, color 0.15s ease;
}
.select-option:hover { background: var(--hover-bg); color: var(--text-primary); }
.select-option-active { background: var(--accent-bg); color: var(--accent-text); font-weight: 600; }
.select-option-active:hover { background: var(--accent-bg); }

/* Drawer 抽屜 */
.drawer-panel {
  background: var(--bg-drawer); color: var(--text-primary);
}

/* Dropdown 過渡 */
.dropdown-enter-active, .dropdown-leave-active { transition: opacity 0.15s ease, transform 0.15s ease; transform-origin: top; }
.dropdown-enter-from, .dropdown-leave-to { opacity: 0; transform: scaleY(0.9) translateY(-2px); }

/* Drawer 過渡 */
.drawer-enter-active, .drawer-leave-active { transition: transform 0.25s ease; }
.drawer-enter-from, .drawer-leave-to { transform: translateY(100%); }
.drawer-overlay-enter-active, .drawer-overlay-leave-active { transition: opacity 0.25s ease; }
.drawer-overlay-enter-from, .drawer-overlay-leave-to { opacity: 0; }

/* List 過渡 */
.list-enter-active {
  transition: all 0.3s ease;
}
.list-leave-active {
  transition: all 0.25s ease;
}
.list-enter-from {
  opacity: 0;
  transform: translateY(-12px);
}
.list-leave-to {
  opacity: 0;
  transform: translateX(20px);
}
.list-move {
  transition: transform 0.3s ease;
}

/* 深色模式圖示反轉 */
html.dark .icon-adapt { filter: invert(1); }

/* Fade 過渡 */
.fade-enter-active, .fade-leave-active { transition: opacity 0.15s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
