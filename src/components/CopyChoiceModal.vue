<template>
  <Teleport to="body">
    <Transition name="modal" appear>
      <div v-if="show" class="copy-choice-modal-overlay" @click="$emit('close')">
        <div class="copy-choice-modal" @click.stop>
          <div class="modal-header">
            <h3>{{ $t('copyChoice.title') }}</h3>
            <button @click="$emit('close')" class="close-btn">×</button>
          </div>
          <div class="modal-body">
            <button @click="handleChoice('email')" class="choice-btn">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                <path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.89 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"/>
              </svg>
              <span>{{ $t('copyChoice.copyEmail') }}</span>
            </button>
            <button @click="handleChoice('session')" class="choice-btn">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>
              </svg>
              <span>{{ $t('copyChoice.copySession') }}</span>
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
const props = defineProps({
  show: Boolean
})

const emit = defineEmits(['close', 'choice'])

const handleChoice = (type) => {
  emit('choice', type)
  emit('close')
}
</script>

<style scoped>
.copy-choice-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 3000;
}

.copy-choice-modal {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  width: 90%;
  max-width: 400px;
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary, #374151);
}

.close-btn {
  background: none;
  border: none;
  font-size: 28px;
  color: var(--color-text-secondary, #6b7280);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-primary, #374151);
}

.modal-body {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.choice-btn {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 20px;
  background: var(--color-surface, #ffffff);
  border: 2px solid var(--color-divider, #e1e5e9);
  border-radius: 8px;
  cursor: pointer;
  font-size: 16px;
  font-weight: 500;
  color: var(--color-text-primary, #374151);
  transition: all 0.2s ease;
}

.choice-btn:hover {
  background: var(--color-primary-surface, #eff6ff);
  border-color: var(--color-primary, #3b82f6);
  color: var(--color-primary, #3b82f6);
}

.choice-btn svg {
  flex-shrink: 0;
}

.choice-btn span {
  flex: 1;
  text-align: left;
}

/* 动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .copy-choice-modal,
.modal-leave-active .copy-choice-modal {
  transition: transform 0.3s ease;
}

.modal-enter-from .copy-choice-modal,
.modal-leave-to .copy-choice-modal {
  transform: scale(0.9);
}
</style>

