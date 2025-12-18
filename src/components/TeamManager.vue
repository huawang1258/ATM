<template>
  <Teleport to="body">
    <Transition name="modal" appear>
      <div v-if="show" class="team-modal-overlay" @click="$emit('close')">
        <div class="team-modal" @click.stop>
          <div class="modal-header">
            <h3>{{ $t('teamManager.title') }}</h3>
            <div class="header-actions">
              <button @click="refreshSession" :disabled="isRefreshing" class="btn-refresh" :title="$t('teamManager.refreshSession')">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" :class="{ 'spinning': isRefreshing }">
                  <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
                </svg>
              </button>
              <button @click="$emit('close')" class="modal-close">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
                </svg>
              </button>
            </div>
          </div>
          
          <div class="modal-body">
            <!-- 加载状态 -->
            <div v-if="isLoading" class="loading-state">
              <div class="spinner"></div>
              <p>{{ $t('teamManager.loading') }}</p>
            </div>

            <!-- 团队内容(包括有团队和无团队的情况) -->
            <div v-else class="team-content">
              <!-- 无团队提示 -->
              <div v-if="!teamInfo || !teamInfo.team" class="no-team-notice">
                <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor" style="opacity: 0.5;">
                  <path d="M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z"/>
                </svg>
                <p style="margin: 12px 0; opacity: 0.7;">{{ $t('teamManager.noTeam') }}</p>
                <p style="margin: 0; font-size: 13px; opacity: 0.5;">{{ $t('teamManager.canStillInvite') }}</p>
              </div>

              <!-- 团队统计(仅在有团队时显示) -->
              <div v-if="teamInfo && teamInfo.team" class="team-stats">
                <div class="stat-item">
                  <span class="stat-label">{{ $t('teamManager.totalSeats') }}</span>
                  <span class="stat-value">{{ teamInfo.team.seats }}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">{{ $t('teamManager.usedSeats') }}</span>
                  <span class="stat-value">{{ teamInfo.team.users.length }}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">{{ $t('teamManager.pendingInvites') }}</span>
                  <span class="stat-value">{{ teamInfo.team.invitations.length }}</span>
                </div>
              </div>

              <!-- 邀请成员(始终显示) -->
              <div class="invite-section">
                <h4>{{ $t('teamManager.inviteMembers') }}</h4>
                <div class="invite-form">
                  <input
                    v-model="inviteEmail"
                    type="email"
                    :placeholder="$t('teamManager.emailPlaceholder')"
                    @keyup.enter="inviteMember"
                  />
                  <button @click="inviteMember" :disabled="!inviteEmail || isInviting" class="btn primary">
                    {{ isInviting ? $t('teamManager.inviting') : $t('teamManager.invite') }}
                  </button>
                </div>
              </div>

              <!-- 已入团成员(仅在有团队时显示) -->
              <div v-if="teamInfo && teamInfo.team && teamInfo.team.users.length > 0" class="members-section">
                <h4>{{ $t('teamManager.members') }} ({{ teamInfo.team.users.length }})</h4>
                <div class="members-list">
                  <div v-for="user in teamInfo.team.users" :key="user.id" class="member-item">
                    <div class="member-info">
                      <div class="member-avatar">
                        {{ user.email.charAt(0).toUpperCase() }}
                      </div>
                      <div class="member-details">
                        <div class="member-email">{{ user.email }}</div>
                        <div class="member-meta">
                          <span class="member-role" :class="user.role.toLowerCase()">{{ user.role }}</span>
                          <span class="member-joined">{{ $t('teamManager.joined') }}: {{ formatDate(user.joinedAt) }}</span>
                        </div>
                      </div>
                    </div>
                    <button
                      v-if="user.role !== 'ADMIN'"
                      @click="removeMember(user)"
                      class="btn-remove"
                      :title="$t('teamManager.removeMember')"
                    >
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
                      </svg>
                    </button>
                  </div>
                </div>
              </div>

              <!-- 待入团邀请(仅在有团队时显示) -->
              <div v-if="teamInfo && teamInfo.team && teamInfo.team.invitations.length > 0" class="invitations-section">
                <h4>{{ $t('teamManager.pendingInvitations') }} ({{ teamInfo.team.invitations.length }})</h4>
                <div class="invitations-list">
                  <div v-for="invitation in teamInfo.team.invitations" :key="invitation.id" class="invitation-item">
                    <div class="invitation-info">
                      <div class="invitation-avatar">
                        {{ invitation.email.charAt(0).toUpperCase() }}
                      </div>
                      <div class="invitation-details">
                        <div class="invitation-email">{{ invitation.email }}</div>
                        <div class="invitation-date">{{ $t('teamManager.invitedAt') }}: {{ formatDate(invitation.invitedAt) }}</div>
                      </div>
                    </div>
                    <button
                      @click="cancelInvitation(invitation)"
                      class="btn-cancel"
                      :title="$t('teamManager.cancelInvitation')"
                    >
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps({
  show: {
    type: Boolean,
    required: true
  },
  authSession: {
    type: String,
    required: true
  }
})

const emit = defineEmits(['close'])

const teamInfo = ref(null)
const isLoading = ref(false)
const error = ref(null)
const inviteEmail = ref('')
const isInviting = ref(false)
const isRefreshing = ref(false)

// 加载团队信息
const loadTeamInfo = async () => {
  isLoading.value = true

  try {
    const result = await invoke('team_get_info', {
      authSession: props.authSession
    })

    // 无论是否有团队信息,都保存结果
    teamInfo.value = result

    // 如果没有团队信息,只是提示,不阻止操作
    if (result.status === 'none' || !result.team) {
      console.log('该账号暂无团队信息,但仍可发送邀请')
    }
  } catch (err) {
    console.error('加载团队信息失败:', err)
    window.$notify.error(`${t('teamManager.loadFailed')}: ${err}`)
    // 即使加载失败,也设置一个空的 teamInfo,允许用户尝试邀请
    teamInfo.value = { status: 'none', team: null }
  } finally {
    isLoading.value = false
  }
}

// 邀请成员
const inviteMember = async () => {
  if (!inviteEmail.value) return
  
  isInviting.value = true
  
  try {
    await invoke('team_invite_members', {
      authSession: props.authSession,
      emails: [inviteEmail.value]
    })
    
    window.$notify.success(t('teamManager.inviteSuccess'))
    inviteEmail.value = ''
    
    // 重新加载团队信息
    await loadTeamInfo()
  } catch (err) {
    window.$notify.error(`${t('teamManager.inviteFailed')}: ${err}`)
  } finally {
    isInviting.value = false
  }
}

// 移除成员
const removeMember = async (user) => {
  if (!confirm(t('teamManager.confirmRemove', { email: user.email }))) {
    return
  }

  try {
    await invoke('team_remove_member', {
      authSession: props.authSession,
      userId: user.id
    })

    window.$notify.success(t('teamManager.removeSuccess'))

    // 重新加载团队信息
    await loadTeamInfo()
  } catch (err) {
    window.$notify.error(`${t('teamManager.removeFailed')}: ${err}`)
  }
}

// 取消邀请
const cancelInvitation = async (invitation) => {
  if (!confirm(t('teamManager.confirmCancel', { email: invitation.email }))) {
    return
  }

  try {
    await invoke('team_cancel_invitation', {
      authSession: props.authSession,
      invitationId: invitation.id
    })

    window.$notify.success(t('teamManager.cancelSuccess'))

    // 重新加载团队信息
    await loadTeamInfo()
  } catch (err) {
    window.$notify.error(`${t('teamManager.cancelFailed')}: ${err}`)
  }
}

// 格式化日期
const formatDate = (dateString) => {
  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 刷新 session
const refreshSession = async () => {
  isRefreshing.value = true

  try {
    const newSession = await invoke('team_refresh_session', {
      authSession: props.authSession
    })

    console.log('Session 刷新成功,新 session 长度:', newSession.length)
    window.$notify.success(t('teamManager.refreshSuccess'))

    // 刷新后重新加载团队信息
    await loadTeamInfo()
  } catch (err) {
    console.error('Session 刷新失败:', err)
    window.$notify.error(`${t('teamManager.refreshFailed')}: ${err}`)
  } finally {
    isRefreshing.value = false
  }
}

// 检查 session 有效性
const checkSessionValidity = async () => {
  try {
    const isValid = await invoke('team_check_session_validity', {
      authSession: props.authSession
    })

    if (isValid) {
      console.log('Session 有效')
      window.$notify.success(t('teamManager.sessionValid'))
    } else {
      console.log('Session 已失效')
      window.$notify.warning(t('teamManager.sessionInvalid'))
    }

    return isValid
  } catch (err) {
    console.error('检查 session 失败:', err)
    return false
  }
}

// 监听 show 属性变化,打开时加载团队信息
watch(() => props.show, (newValue) => {
  if (newValue) {
    loadTeamInfo()
  }
})
</script>

<style scoped>
/* 模态框样式 */
.team-modal-overlay {
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
  padding: 20px;
}

.team-modal {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  width: 100%;
  max-width: 700px;
  max-height: 90vh;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
  background: var(--color-surface-alt, #f9fafb);
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-heading, #1f2937);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-refresh {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--color-text-secondary, #6b7280);
  padding: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.btn-refresh:hover:not(:disabled) {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-primary, #3b82f6);
}

.btn-refresh:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-refresh svg.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.modal-close {
  background: none;
  border: none;
  cursor: pointer;
  color: var(--color-text-secondary, #6b7280);
  padding: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s;
}

.modal-close:hover {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-primary, #111827);
}

.modal-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

/* 加载和错误状态 */
.loading-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 16px;
}

/* 无团队提示 */
.no-team-notice {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px 20px;
  gap: 8px;
  background: var(--color-surface-alt, #f9fafb);
  border-radius: 8px;
  margin-bottom: 24px;
  border: 1px dashed var(--color-border, #e5e7eb);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--color-border, #e5e7eb);
  border-top-color: var(--color-primary, #3b82f6);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-state svg {
  color: var(--color-danger, #ef4444);
}

/* 团队统计 */
.team-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 16px;
  margin-bottom: 24px;
}

.stat-item {
  background: var(--color-surface-alt, #f9fafb);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 8px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.stat-label {
  font-size: 13px;
  color: var(--color-text-secondary, #6b7280);
}

.stat-value {
  font-size: 24px;
  font-weight: 600;
  color: var(--color-primary, #3b82f6);
}

/* 邀请区域 */
.invite-section {
  margin-bottom: 24px;
  padding: 20px;
  background: var(--color-surface-alt, #f9fafb);
  border-radius: 8px;
  border: 1px solid var(--color-border, #e5e7eb);
}

.invite-section h4 {
  margin: 0 0 12px 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-heading, #1f2937);
}

.invite-form {
  display: flex;
  gap: 12px;
}

.invite-form input {
  flex: 1;
  padding: 10px 14px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
  font-size: 14px;
  background: var(--color-surface, #ffffff);
  color: var(--color-text-primary, #111827);
}

.invite-form input:focus {
  outline: none;
  border-color: var(--color-primary, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

/* 成员列表 */
.members-section,
.invitations-section {
  margin-bottom: 24px;
}

.members-section h4,
.invitations-section h4 {
  margin: 0 0 12px 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-heading, #1f2937);
}

.members-list,
.invitations-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.member-item,
.invitation-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--color-surface, #ffffff);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 8px;
  transition: all 0.2s;
}

.member-item:hover,
.invitation-item:hover {
  border-color: var(--color-primary, #3b82f6);
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.1);
}

.member-info,
.invitation-info {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.member-avatar,
.invitation-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 16px;
  flex-shrink: 0;
}

.member-details,
.invitation-details {
  flex: 1;
  min-width: 0;
}

.member-email,
.invitation-email {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary, #111827);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.member-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 4px;
}

.member-role {
  font-size: 12px;
  padding: 2px 8px;
  border-radius: 4px;
  font-weight: 500;
}

.member-role.admin {
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
}

.member-role.member {
  background: rgba(59, 130, 246, 0.1);
  color: #2563eb;
}

.member-joined,
.invitation-date {
  font-size: 12px;
  color: var(--color-text-secondary, #6b7280);
}

.invitation-status {
  font-size: 12px;
  padding: 4px 12px;
  border-radius: 12px;
  background: rgba(251, 191, 36, 0.1);
  color: #f59e0b;
  font-weight: 500;
}

.btn-remove,
.btn-cancel {
  background: transparent;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
  padding: 8px;
  cursor: pointer;
  color: var(--color-danger, #ef4444);
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-remove:hover,
.btn-cancel:hover {
  background: rgba(239, 68, 68, 0.1);
  border-color: var(--color-danger, #ef4444);
}

/* 按钮样式 */
.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.btn.primary {
  background: var(--color-primary, #3b82f6);
  color: white;
}

.btn.primary:hover:not(:disabled) {
  background: #2563eb;
}

.btn.primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 过渡动画 */
.modal-enter-active,
.modal-leave-active {
  transition: all 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>

