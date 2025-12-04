<template>
  <div class="token-list-modal">
    <div class="modal-overlay">
      <div class="modal-content" @click.stop="handleModalContentClick">
        <div class="modal-header">
          <div class="header-title">
            <h2>{{ $t('tokenList.title') }}</h2>
            <div :class="['status-badge', storageStatusClass]">
              <span :class="['status-dot', storageStatusClass]"></span>
              <span class="status-text">{{ storageStatusText }}</span>
            </div>
          </div>
          <div class="header-actions">
            <!-- 检测重复按钮 - 始终显示 -->
            <button
              @click="duplicateInfo.hasDuplicates ? showDuplicateModal = true : detectDuplicates()"
              :class="['btn', 'small', duplicateInfo.hasDuplicates ? 'warning' : 'secondary']"
              :title="duplicateInfo.hasDuplicates ? `发现 ${duplicateInfo.duplicateCount} 条重复token，点击查看详情` : '点击检测重复token'"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
              </svg>
              {{ duplicateInfo.hasDuplicates ? `检测到重复 (${duplicateInfo.duplicateCount})` : '🔍 检测重复' }}
            </button>
            <!-- 批量获取额度按钮 -->
            <button
              @click="batchGetCredits"
              :class="['btn', 'small', 'success']"
              :disabled="isBatchGettingCredits || tokensNeedingCredits === 0"
              :title="tokensNeedingCredits > 0 ? `批量获取 ${tokensNeedingCredits} 个token的额度` : '没有需要获取额度的token'"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z"/>
              </svg>
              {{ isBatchGettingCredits ? '获取中...' : `批量获取额度 (${tokensNeedingCredits})` }}
            </button>
            <!-- 数据库配置按钮 -->
            <button @click="showDatabaseConfig = true" class="btn info small">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 3C7.58 3 4 4.79 4 7s3.58 4 8 4 8-1.79 8-4-3.58-4-8-4zM4 9v3c0 2.21 3.58 4 8 4s8-1.79 8-4V9c0 2.21-3.58 4-8 4s-8-1.79-8-4zM4 16v3c0 2.21 3.58 4 8 4s8-1.79 8-4v-3c0 2.21-3.58 4-8 4s-8-1.79-8-4z"/>
              </svg>
              {{ $t('tokenList.databaseConfig') }}
            </button>
            <!-- 手动保存按钮 -->
            <button
              @click="handleManualSave"
              class="btn success small"
              :disabled="isSaving"
              title="手动保存到本地JSON文件"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"/>
              </svg>
              {{ isSaving ? '保存中...' : '保存' }}
            </button>

            <!-- 强制推送到数据库按钮 - 仅双向存储模式显示 -->
            <button
              v-if="isDatabaseAvailable"
              @click="handleForcePushToDatabase"
              class="btn warning small"
              :disabled="isForcePushing"
              title="强制推送本地所有数据到数据库（覆盖数据库数据）"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2L4 5v6.09c0 5.05 3.41 9.76 8 10.91 4.59-1.15 8-5.86 8-10.91V5l-8-3zm6 9.09c0 4-2.55 7.7-6 8.83-3.45-1.13-6-4.82-6-8.83V6.31l6-2.12 6 2.12v4.78z"/>
                <path d="M10.5 12.5l-2-2-1.41 1.41L10.5 15.5 17 9l-1.41-1.41z"/>
              </svg>
              {{ isForcePushing ? '推送中...' : '强推数据库' }}
            </button>

            <!-- 同步按钮 - 仅双向存储模式显示 -->
            <button
              v-if="isDatabaseAvailable"
              @click="handleBidirectionalSync"
              class="btn info small"
              :disabled="isSyncing"
              :title="$t('tokenList.syncTooltip')"
            >
              <svg v-if="!isSyncing" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z"/>
              </svg>
              {{ isSyncing ? $t('tokenList.syncing') : $t('tokenList.sync') }}
            </button>
            <button @click="handleAddToken" class="btn primary small">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
              </svg>
              {{ $t('tokenList.addToken') }}
            </button>
            <button @click="handleRefresh" class="btn secondary small" :disabled="isRefreshing">
              <svg v-if="!isRefreshing" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17.65 6.35C16.2 4.9 14.21 4 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08c-.82 2.33-3.04 4-5.65 4-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
              </svg>
              {{ isRefreshing ? $t('loading.refreshing') : $t('tokenList.refresh') }}
            </button>
            <!-- 多选模式切换按钮 -->
            <button
              @click="toggleSelectionMode"
              :class="['btn', 'small', selectionMode ? 'primary' : 'secondary']"
              :title="selectionMode ? '关闭多选模式' : '开启多选模式'"
            >
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2zm-9 14l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
              </svg>
              {{ selectionMode ? '退出多选' : '多选' }}
            </button>
            <button class="close-btn" @click="handleClose">×</button>
          </div>
        </div>

        <!-- 批量操作区域 -->
        <div v-if="selectionMode && selectedCount > 0" class="batch-actions-bar">
          <div class="batch-info">
            <span class="selected-count">已选中 {{ selectedCount }} 个</span>
          </div>
          <div class="batch-buttons">
            <button @click="toggleSelectAll" class="btn small secondary">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2zm-9 14l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
              </svg>
              {{ isCurrentPageAllSelected ? '取消全选' : '全选当前页' }}
            </button>
            <button @click="copySelectedEmails" class="btn small info">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.89 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"/>
              </svg>
              复制邮箱
            </button>
            <button @click="copySelectedSessions" class="btn small info">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>
              </svg>
              复制Session
            </button>
            <button @click="exportSelectedTokens" class="btn small success">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 12v7H5v-7H3v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-7h-2zm-6 .67l2.59-2.58L17 11.5l-5 5-5-5 1.41-1.41L11 12.67V3h2z"/>
              </svg>
              导出选中
            </button>
            <button @click="deleteSelectedTokens" class="btn small danger">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z"/>
              </svg>
              删除选中
            </button>
            <button @click="clearSelection" class="btn small secondary">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
              </svg>
              取消选中
            </button>
          </div>
        </div>
        
        <div class="modal-body">
          <!-- Loading State -->
          <div v-if="isLoading" class="loading-state">
            <div class="spinner"></div>
            <p>{{ $t('tokenList.loading') }}</p>
          </div>

          <!-- Empty State -->
          <div v-else-if="tokens.length === 0" class="empty-state">
            <div class="empty-icon">
              <svg width="64" height="64" viewBox="0 0 24 24" fill="currentColor">
                <path
                  d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" />
              </svg>
            </div>
            <h3>{{ $t('tokenList.empty') }}</h3>
            <button class="batch-import-btn-empty" @click="showBatchImportConfirm" :title="$t('tokenList.batchImport')">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                <path
                  d="M19 12v7H5v-7H3v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-7h-2zm-6 .67l2.59-2.58L17 11.5l-5 5-5-5 1.41-1.41L11 12.67V3h2z" />
              </svg>
              {{ $t('tokenList.batchImport') }}
            </button>
          </div>

          <!-- Token List -->
          <div v-else class="token-list">
            <!-- 统计概览卡片 -->
            <div class="stats-overview">
              <div class="stat-card total" @click="quickFilterByStatus('all')" :title="$t('tokenList.quickFilter')">
                <div class="stat-label">{{ $t('tokenList.statsTotal') }}</div>
                <div class="stat-value">{{ tokenStats.total }}</div>
              </div>
              <div class="stat-card normal" @click="quickFilterByStatus('normal')" :title="$t('tokenList.quickFilter')">
                <div class="stat-label">{{ $t('tokenList.statsNormal') }}</div>
                <div class="stat-value">{{ tokenStats.normal }}</div>
              </div>
              <div class="stat-card abnormal" @click="quickFilterByStatus('abnormal')" :title="$t('tokenList.quickFilter')">
                <div class="stat-label">{{ $t('tokenList.statsBanned') }}</div>
                <div class="stat-value">{{ tokenStats.abnormal }}</div>
              </div>
              <!-- 按额度分类统计 -->
              <div class="stat-card credits-below-4000" @click="quickFilterByCredits('below4000')" :title="$t('tokenList.quickFilter')">
                <div class="stat-label">4000以下</div>
                <div class="stat-value">{{ tokenStats.creditsBelow4000 }}</div>
              </div>
              <div class="stat-card credits-exact-4000" @click="quickFilterByCredits('exact4000')" :title="$t('tokenList.quickFilter')">
                <div class="stat-label">恰好4000</div>
                <div class="stat-value">{{ tokenStats.creditsExact4000 }}</div>
              </div>
              <div class="stat-card credits-between-4001-34000" @click="quickFilterByCredits('between4001And34000')" :title="$t('tokenList.quickFilter')">
                <div class="stat-label">4001-34000</div>
                <div class="stat-value">{{ tokenStats.creditsBetween4001And34000 }}</div>
              </div>
            </div>

            <div class="list-header">
              <div class="list-toolbar">
                <!-- 搜索框 (移到最前面) -->
                <div class="search-box">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="search-icon">
                    <path
                      d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z" />
                  </svg>
                  <input type="text" v-model="searchQuery" :placeholder="$t('tokenList.searchPlaceholder')"
                    class="search-input" />
                  <button v-if="searchQuery.trim()" @click="searchQuery = ''" class="clear-search-btn" title="清空搜索">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                      <path
                        d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" />
                    </svg>
                  </button>
                </div>

                <!-- 排序下拉菜单 -->
                <div class="sort-dropdown">
                  <button class="sort-btn" @click.stop="toggleSortMenu" :title="$t('tokenList.sort')">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                      <!-- 左边向上箭头 -->
                      <path d="M7 16V6M4 9l3-3 3 3"/>
                      <!-- 右边向下箭头 -->
                      <path d="M17 8v10M14 15l3 3 3-3"/>
                    </svg>
                  </button>

                  <!-- 下拉菜单 -->
                  <Transition name="dropdown">
                    <div v-if="showSortMenu" class="sort-menu" @click.stop>
                      <button
                        :class="['sort-option', { active: sortType === 'time' && sortOrder === 'desc' }]"
                        @click="setSortType('time', 'desc')"
                      >
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                          <path d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z"/>
                        </svg>
                        <span>{{ $t('tokenList.sortByTime') }}</span>
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" class="arrow-down">
                          <path d="M16 10l-4 4-4-4h8z"/>
                        </svg>
                        <svg v-if="sortType === 'time' && sortOrder === 'desc'" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="check-icon">
                          <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                        </svg>
                      </button>

                      <button
                        :class="['sort-option', { active: sortType === 'time' && sortOrder === 'asc' }]"
                        @click="setSortType('time', 'asc')"
                      >
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                          <path d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z"/>
                        </svg>
                        <span>{{ $t('tokenList.sortByTime') }}</span>
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" class="arrow-up">
                          <path d="M8 14l4-4 4 4H8z"/>
                        </svg>
                        <svg v-if="sortType === 'time' && sortOrder === 'asc'" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="check-icon">
                          <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                        </svg>
                      </button>

                      <div class="sort-divider"></div>

                      <button
                        :class="['sort-option', { active: sortType === 'balance' && sortOrder === 'desc' }]"
                        @click="setSortType('balance', 'desc')"
                      >
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                          <path d="M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z"/>
                        </svg>
                        <span>{{ $t('tokenList.sortByBalance') }}</span>
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" class="arrow-down">
                          <path d="M16 10l-4 4-4-4h8z"/>
                        </svg>
                        <svg v-if="sortType === 'balance' && sortOrder === 'desc'" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="check-icon">
                          <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                        </svg>
                      </button>

                      <button
                        :class="['sort-option', { active: sortType === 'balance' && sortOrder === 'asc' }]"
                        @click="setSortType('balance', 'asc')"
                      >
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                          <path d="M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.78 0 2.44.85 2.5 2.1h2.21c-.07-1.72-1.12-3.3-3.21-3.81V3h-3v2.16c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-2.06 0-2.87-.92-2.98-2.1h-2.2c.12 2.19 1.76 3.42 3.68 3.83V21h3v-2.15c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z"/>
                        </svg>
                        <span>{{ $t('tokenList.sortByBalance') }}</span>
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" class="arrow-up">
                          <path d="M8 14l4-4 4 4H8z"/>
                        </svg>
                        <svg v-if="sortType === 'balance' && sortOrder === 'asc'" width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="check-icon">
                          <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
                        </svg>
                      </button>
                    </div>
                  </Transition>
                </div>

                <!-- 过滤按钮 -->
                <div class="filter-dropdown">
                  <button
                    class="filter-btn"
                    :class="{ active: filterMode !== 'all' }"
                    @click.stop="showFilterMenu = !showFilterMenu"
                    :title="$t('tokenList.filter')"
                  >
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                      <path d="M3 4a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v2.586a1 1 0 0 1-.293.707l-6.414 6.414a1 1 0 0 0-.293.707V17l-4 4v-6.586a1 1 0 0 0-.293-.707L3.293 7.293A1 1 0 0 1 3 6.586V4z"/>
                    </svg>
                    <span>{{ filterModeLabel }}</span>
                  </button>

                  <!-- 筛选下拉菜单 -->
                  <Transition name="dropdown">
                    <div v-if="showFilterMenu" class="filter-menu" @click.stop>
                      <div class="filter-menu-item" :class="{ active: filterMode === 'all' }" @click="setFilterMode('all')">
                        <span class="filter-icon">📋</span>
                        <span>{{ $t('tokenList.filterAll') }}</span>
                      </div>
                      <div class="filter-menu-item" :class="{ active: filterMode === 'normal' }" @click="setFilterMode('normal')">
                        <span class="filter-icon">✅</span>
                        <span>{{ $t('tokenList.filterNormal') }}</span>
                      </div>
                      <div class="filter-menu-item" :class="{ active: filterMode === 'abnormal' }" @click="setFilterMode('abnormal')">
                        <span class="filter-icon">⚠️</span>
                        <span>{{ $t('tokenList.filterAbnormal') }}</span>
                      </div>
                      <div class="filter-menu-item" :class="{ active: filterMode === 'bindcard' }" @click="setFilterMode('bindcard')">
                        <span class="filter-icon">💳</span>
                        <span>{{ $t('tokenList.filterBindCard') }}</span>
                      </div>
                      <div class="filter-menu-item" :class="{ active: filterMode === 'unbindcard' }" @click="setFilterMode('unbindcard')">
                        <span class="filter-icon">🔓</span>
                        <span>{{ $t('tokenList.filterUnbindCard') }}</span>
                      </div>
                    </div>
                  </Transition>
                </div>

                <!-- 余额筛选按钮 -->
                <div class="balance-filter-dropdown">
                  <button
                    class="balance-filter-btn"
                    :class="{ active: balanceFilterEnabled }"
                    @click.stop="showBalanceFilterPanel = !showBalanceFilterPanel"
                    :title="$t('tokenList.balanceFilter')"
                  >
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                      <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 6.5c2.33 0 4.31-1.46 5.11-3.5H6.89c.8 2.04 2.78 3.5 5.11 3.5z"/>
                    </svg>
                    <span>{{ $t('tokenList.balanceFilter') }}</span>
                    <span v-if="balanceFilterEnabled" class="filter-badge">✓</span>
                  </button>

                  <!-- 余额筛选面板 -->
                  <Transition name="dropdown">
                    <div v-if="showBalanceFilterPanel" class="balance-filter-panel" @click.stop>
                      <div class="filter-panel-header">
                        <h4>{{ $t('tokenList.balanceFilterTitle') }}</h4>
                      </div>
                      <div class="filter-panel-body">
                        <div class="filter-input-group">
                          <label>{{ $t('tokenList.balanceFilterMin') }}</label>
                          <input
                            v-model.number="balanceFilterMin"
                            type="number"
                            :placeholder="$t('tokenList.balanceFilterMinPlaceholder')"
                            class="filter-input"
                            min="0"
                          />
                        </div>
                        <div class="filter-input-group">
                          <label>{{ $t('tokenList.balanceFilterMax') }}</label>
                          <input
                            v-model.number="balanceFilterMax"
                            type="number"
                            :placeholder="$t('tokenList.balanceFilterMaxPlaceholder')"
                            class="filter-input"
                            min="0"
                          />
                        </div>
                      </div>
                      <div class="filter-panel-footer">
                        <button @click="clearBalanceFilter" class="btn secondary small">
                          {{ $t('tokenList.balanceFilterClear') }}
                        </button>
                        <button @click="applyBalanceFilter" class="btn primary small">
                          {{ $t('tokenList.balanceFilterApply') }}
                        </button>
                      </div>
                    </div>
                  </Transition>
                </div>

                <!-- 其他按钮 -->
                <button class="open-folder-btn" @click="openDataFolder" :title="$t('bookmarkManager.openDataFolder')">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <path
                      d="M10 4H4c-1.11 0-2 .89-2 2v12c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2h-8l-2-2z" />
                  </svg>
                </button>
                <button class="batch-import-btn" @click="showBatchImportConfirm" :title="$t('tokenList.batchImport')">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
                  </svg>
                </button>
                <button class="batch-delete-btn" @click="handleBatchDelete" :title="$t('tokenList.batchDelete')">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z" />
                  </svg>
                </button>
                <button class="export-btn" @click="handleExportTokens" :title="$t('tokenList.export')">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M19 12v7H5v-7H3v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-7h-2zm-6 .67l2.59-2.58L17 11.5l-5 5-5-5 1.41-1.41L11 12.67V3h2z" />
                  </svg>
                </button>
                <button class="copy-emails-btn" @click="copyFilteredEmails" :title="$t('tokenList.copyEmails')" :disabled="filteredTokens.length === 0">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z"/>
                  </svg>
                </button>

                <!-- 分页信息和每页数量 -->
                <div v-if="filteredTokens.length > 0" class="pagination-combined">
                  <span class="pagination-info-text">{{ $t('pagination.page', { current: currentPage, total: filteredTokens.length }) }}</span>
                  <select v-model.number="pageSize" @change="changePageSize(pageSize)" class="pagination-size-select">
                    <option v-for="size in pageSizeOptions" :key="size" :value="size">
                      {{ size }}
                    </option>
                  </select>
                </div>
              </div>
            </div>

            <!-- 无搜索结果提示 -->
            <div v-if="searchQuery.trim() && filteredTokens.length === 0" class="no-search-results">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="currentColor" opacity="0.3">
                <path
                  d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z" />
              </svg>
              <p>{{ $t('tokenList.noSearchResults') }}</p>
            </div>

            <template v-else>
              <!-- Token Grid -->
              <div class="token-grid">
                <TokenCard
                  v-for="token in paginatedTokens"
                  :key="token.id"
                  :ref="el => setTokenCardRef(el, token.id)"
                  :token="token"
                  :is-batch-checking="isRefreshing"
                  :is-highlighted="highlightedTokenId === token.id"
                  :selection-mode="selectionMode"
                  :is-selected="selectedTokenIds.has(token.id)"
                  @delete="deleteToken"
                  @edit="handleEditToken"
                  @token-updated="handleTokenUpdated"
                  @toggle-selection="toggleTokenSelection"
                />
              </div>

              <!-- 分页导航 -->
              <div v-if="totalPages > 1" class="pagination-nav">
                <button
                  class="pagination-btn"
                  :disabled="currentPage === 1"
                  @click="prevPage"
                >
                  {{ $t('pagination.prev') }}
                </button>

                <div class="pagination-pages">
                  <!-- 第一页 -->
                  <button
                    v-if="showFirstPage"
                    :class="['page-number', { active: currentPage === 1 }]"
                    @click="goToPage(1)"
                  >
                    1
                  </button>

                  <!-- 左侧省略号 -->
                  <span v-if="showLeftEllipsis" class="page-ellipsis">...</span>

                  <!-- 中间页码 -->
                  <button
                    v-for="page in visiblePages"
                    :key="page"
                    :class="['page-number', { active: currentPage === page }]"
                    @click="goToPage(page)"
                  >
                    {{ page }}
                  </button>

                  <!-- 右侧省略号 -->
                  <span v-if="showRightEllipsis" class="page-ellipsis">...</span>

                  <!-- 最后一页 -->
                  <button
                    v-if="showLastPage"
                    :class="['page-number', { active: currentPage === totalPages }]"
                    @click="goToPage(totalPages)"
                  >
                    {{ totalPages }}
                  </button>
                </div>

                <!-- 页码跳转 -->
                <div class="page-jump">
                  <span>跳转</span>
                  <input
                    v-model.number="jumpToPageInput"
                    type="number"
                    min="1"
                    :max="totalPages"
                    class="page-jump-input"
                    @keyup.enter="handleJumpToPage"
                  />
                  <button class="pagination-btn" @click="handleJumpToPage">GO</button>
                </div>

                <button
                  class="pagination-btn"
                  :disabled="currentPage === totalPages"
                  @click="nextPage"
                >
                  {{ $t('pagination.next') }}
                </button>
              </div>
            </template>

          </div>
        </div>
      </div>
    </div>

    <!-- 直达顶部/底部按钮 -->
    <div class="scroll-buttons">
      <button
        @click="scrollToTop"
        class="scroll-btn scroll-to-top"
        title="直达顶部"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
          <path d="M7.41 15.41L12 10.83l4.59 4.58L18 14l-6-6-6 6z"/>
        </svg>
      </button>
      <button
        @click="scrollToBottom"
        class="scroll-btn scroll-to-bottom"
        title="直达底部"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
          <path d="M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z"/>
        </svg>
      </button>
    </div>

    <!-- Database Config Modal -->
    <DatabaseConfig v-if="showDatabaseConfig" @close="showDatabaseConfig = false"
      @config-saved="handleDatabaseConfigSaved" @config-deleted="handleDatabaseConfigDeleted" />

    <!-- Token Form Modal -->
    <TokenForm v-if="showTokenFormModal" :token="editingToken" @close="closeTokenForm" @success="handleTokenFormSuccess"
      @update-token="handleUpdateToken" @add-token="handleAddTokenFromForm"
      @auto-import-completed="handleAutoImportCompleted" @manual-import-completed="handleManualImportCompleted" />

    <!-- Batch Import Dialog -->
    <Teleport to="body">
      <Transition name="modal" appear>
        <div v-if="showBatchImportDialog" class="batch-import-overlay" @click="showBatchImportDialog = false">
          <div class="batch-import-dialog" @click.stop>
            <div class="dialog-header">
              <h3>{{ $t('tokenList.batchImportTitle') }}</h3>
              <button @click="showBatchImportDialog = false" class="dialog-close">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path
                    d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" />
                </svg>
              </button>
            </div>

            <!-- Tab Navigation -->
            <div class="batch-import-tabs">
              <button :class="['batch-import-tab', { active: batchImportTab === 'session' }]"
                @click="batchImportTab = 'session'">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path
                    d="M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm0 4c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H6v-1.4c0-2 4-3.1 6-3.1s6 1.1 6 3.1V19z" />
                </svg>
                {{ $t('tokenList.sessionImportTab') }}
              </button>
              <button :class="['batch-import-tab', { active: batchImportTab === 'token' }]"
                @click="batchImportTab = 'token'">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z" />
                </svg>
                {{ $t('tokenList.tokenImportTab') }}
              </button>
            </div>

            <div class="dialog-body">
              <!-- Session Tab Content -->
              <div v-if="batchImportTab === 'session'" class="tab-content">
                <!-- 切换按钮：单行输入 vs 多行粘贴 -->
                <div class="session-mode-toggle">
                  <button :class="['mode-btn', { active: sessionImportMode === 'single' }]"
                    @click="sessionImportMode = 'single'">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                      <path d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z" />
                    </svg>
                    单行输入
                  </button>
                  <button :class="['mode-btn', { active: sessionImportMode === 'multi' }]"
                    @click="sessionImportMode = 'multi'">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                      <path d="M4 6h16v2H4zm0 5h16v2H4zm0 5h16v2H4z" />
                    </svg>
                    多行粘贴
                  </button>
                </div>

                <!-- 单行输入模式 -->
                <div v-if="sessionImportMode === 'single'" class="session-single-mode">
                  <p class="dialog-message">{{ $t('tokenList.sessionImportMessage') }}</p>

                  <!-- Session 动态输入框列表 -->
                  <div class="session-inputs-container">
                    <div v-for="(input, index) in sessionInputs" :key="input.id" class="session-input-item">
                      <span class="session-input-number">{{ index + 1 }}.</span>
                      <input v-model="input.value" type="text" :placeholder="$t('tokenList.sessionInputPlaceholder')"
                        class="session-input-field" />
                      <button @click="removeSessionInput(input.id)" class="session-input-delete"
                        :title="$t('tokenList.deleteInput')" :disabled="sessionInputs.length <= 1">
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                          <path d="M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z" />
                        </svg>
                      </button>
                    </div>
                  </div>

                  <!-- 添加更多按钮 -->
                  <button @click="addSessionInput" @contextmenu="handleContextMenu($event, 'session')"
                    class="add-more-btn">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                      <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z" />
                    </svg>
                    {{ $t('tokenList.addMore') }}
                  </button>
                </div>

                <!-- 多行粘贴模式 -->
                <div v-else-if="sessionImportMode === 'multi'" class="session-multi-mode">
                  <p class="dialog-message">在下方粘贴多个 Session，每行一个：</p>
                  <textarea v-model="sessionBatchText" rows="12" class="session-batch-textarea"
                    :placeholder="'session1\nsession2\nsession3\n...'" @input="parseSessionBatch"></textarea>

                  <!-- 预览已识别的 Session 数量 -->
                  <div v-if="parsedSessions.length > 0" class="session-preview-simple">
                    <div class="preview-info">
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" />
                      </svg>
                      <span>已识别 <strong>{{ parsedSessions.length }}</strong> 个有效的 Session</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Token Tab Content -->
              <div v-else-if="batchImportTab === 'token'" class="tab-content">
                <p class="dialog-message">{{ $t('tokenList.tokenImportMessage') }}</p>

                <!-- 格式说明和填充按钮 -->
                <div class="format-option-single">
                  <div class="format-header">
                    <span class="format-title">{{ $t('tokenList.tokenFormatTitle') }}</span>
                  </div>
                  <p class="format-desc">{{ $t('tokenList.tokenFormatDesc') }}</p>
                  <button @click="fillTokenTemplate()" @contextmenu="handleContextMenu($event, 'token')"
                    class="btn-fill-template">
                    {{ $t('tokenList.fillTemplate') }}
                  </button>
                </div>

                <div class="import-input-section">
                  <textarea v-model="importJsonText" rows="10" class="import-textarea"
                    @input="validateImportJson"></textarea>
                </div>

                <!-- 错误信息 -->
                <div v-if="importErrors.length > 0" class="import-errors">
                  <div class="error-header">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                      <path
                        d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z" />
                    </svg>
                    <span>{{ $t('tokenList.importErrorsFound', { count: importErrors.length }) }}</span>
                  </div>
                  <ul class="error-list">
                    <li v-for="(error, index) in importErrors" :key="index">{{ error }}</li>
                  </ul>
                </div>

                <!-- 预览信息 -->
                <div v-if="importPreview.length > 0" class="import-preview">
                  <div class="preview-header">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                      <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" />
                    </svg>
                    <span>{{ $t('tokenList.importPreviewReady', { count: importPreview.length }) }}</span>
                  </div>
                </div>
              </div>
            </div>

            <div class="dialog-footer">
              <button @click="showBatchImportDialog = false" class="btn-cancel">
                {{ $t('tokenList.cancel') }}
              </button>
              <button @click="executeBatchImport" class="btn-confirm"
                :disabled="isImporting || (batchImportTab === 'session' ? getSessionCountForImport() === 0 : importPreview.length === 0)">
                <template v-if="isImporting">
                  {{ $t('tokenList.importing') }}
                </template>
                <template v-else>
                  {{ batchImportTab === 'session'
                    ? $t('tokenList.batchAdd', { count: getSessionCountForImport() })
                    : $t('tokenList.confirmImport')
                  }}
                </template>
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Context Menu for Fill Template -->
    <Teleport to="body">
      <div v-if="showContextMenu" class="context-menu-overlay" @click="closeContextMenu">
        <div class="context-menu" :style="{ left: contextMenuPosition.x + 'px', top: contextMenuPosition.y + 'px' }"
          @click.stop>
          <div class="context-menu-header">{{ $t('tokenList.selectFillCount') }}</div>

          <!-- Session Tab: 简化菜单 -->
          <template v-if="contextMenuType === 'session'">
            <div class="context-menu-custom">
              <input v-model.number="customFillCount" type="number" min="1" max="20"
                :placeholder="$t('tokenList.customCount')" class="custom-count-input" @click.stop />
              <button @click="setDefaultCountFromInput" class="btn-custom-fill">
                {{ $t('tokenList.setAsDefault') }}
              </button>
            </div>
          </template>

          <!-- Token Tab: 完整菜单 -->
          <template v-else>
            <div class="context-menu-custom">
              <input v-model.number="customFillCount" type="number" min="1" max="100"
                :placeholder="$t('tokenList.customCount')" class="custom-count-input" @click.stop />
              <button @click="fillWithCustomCount" class="btn-custom-fill">
                {{ $t('common.confirm') }}
              </button>
            </div>
            <div class="context-menu-divider"></div>
            <div class="context-menu-item" @click="selectFillCount(1)">1</div>
            <div class="context-menu-item" @click="selectFillCount(3)">3</div>
            <div class="context-menu-item" @click="selectFillCount(5)">5</div>
            <div class="context-menu-item" @click="selectFillCount(10)">10</div>
            <div class="context-menu-item" @click="selectFillCount(20)">20</div>
          </template>
        </div>
      </div>
    </Teleport>

    <!-- Batch Delete Confirmation Dialog -->
    <Teleport to="body">
      <Transition name="modal" appear>
        <div v-if="showBatchDeleteDialog" class="batch-delete-overlay" @click="showBatchDeleteDialog = false">
          <div class="batch-delete-dialog" @click.stop>
            <div class="dialog-header">
              <h3>{{ $t('tokenList.batchDeleteConfirm') }}</h3>
              <button @click="showBatchDeleteDialog = false" class="dialog-close">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path
                    d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" />
                </svg>
              </button>
            </div>
            <div class="dialog-body">
              <p class="dialog-message">{{ $t('tokenList.batchDeleteMessage') }}</p>
              <div class="delete-stats">
                <div class="stat-item">
                  <span class="stat-label">{{ $t('tokenList.bannedCount') }}:</span>
                  <span class="stat-value">{{ bannedTokensCount }} {{ $t('tokenList.items') }}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">{{ $t('tokenList.expiredCount') }}:</span>
                  <span class="stat-value">{{ expiredTokensCount }} {{ $t('tokenList.items') }}</span>
                </div>
                <div class="stat-item total">
                  <span class="stat-label">{{ $t('tokenList.totalCount') }}:</span>
                  <span class="stat-value">{{ deletableTokensCount }} {{ $t('tokenList.items') }}</span>
                </div>
              </div>
              <p class="dialog-warning">{{ $t('tokenList.cannotUndo') }}</p>
            </div>
            <div class="dialog-footer">
              <button @click="executeBatchDelete" class="btn btn-danger" :disabled="isDeleting">
                {{ isDeleting ? $t('tokenList.deleting') : $t('tokenList.confirmDelete') }}
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Export Dialog -->
    <Teleport to="body">
      <Transition name="modal" appear>
        <div v-if="showExportDialog" class="export-overlay" @click="showExportDialog = false">
          <div class="export-dialog" @click.stop>
            <div class="dialog-header">
              <h3>{{ $t('tokenList.exportTitle') }}</h3>
              <button @click="showExportDialog = false" class="dialog-close">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                  <path
                    d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" />
                </svg>
              </button>
            </div>
            <div class="dialog-body">
              <p class="dialog-message">{{ $t('tokenList.exportMessage', { total: filteredTokens.length }) }}</p>

              <!-- 导出数量选择 -->
              <div class="export-options">
                <div class="option-group">
                  <label class="option-label">{{ $t('tokenList.exportCount') }}:</label>
                  <div class="export-count-options">
                    <button
                      v-for="count in [10, 20, 50, 100]"
                      :key="count"
                      :class="['count-btn', { active: exportCount === count }]"
                      @click="exportCount = count"
                      :disabled="count > filteredTokens.length"
                    >
                      {{ count }}
                    </button>
                    <button
                      :class="['count-btn', { active: exportCount === 'all' }]"
                      @click="exportCount = 'all'"
                    >
                      {{ $t('tokenList.exportAll') }}
                    </button>
                  </div>
                </div>

                <!-- 导出后删除选项 -->
                <div class="option-group">
                  <label class="option-label">
                    <input type="checkbox" v-model="exportAndDelete" />
                    {{ $t('tokenList.exportAndDelete') }}
                  </label>
                  <p v-if="exportAndDelete" class="option-warning">
                    {{ $t('tokenList.exportAndDeleteWarning') }}
                  </p>
                </div>

                <!-- 导出统计 -->
                <div class="export-stats">
                  <div class="stat-item">
                    <span class="stat-label">{{ $t('tokenList.willExport') }}:</span>
                    <span class="stat-value">{{ getExportCount() }} {{ $t('tokenList.items') }}</span>
                  </div>
                  <div v-if="exportAndDelete" class="stat-item warning">
                    <span class="stat-label">{{ $t('tokenList.willDelete') }}:</span>
                    <span class="stat-value">{{ getExportCount() }} {{ $t('tokenList.items') }}</span>
                  </div>
                </div>
              </div>
            </div>
            <div class="dialog-footer">
              <button @click="showExportDialog = false" class="btn secondary">
                {{ $t('common.cancel') }}
              </button>
              <button @click="executeExport" class="btn primary" :disabled="isExporting">
                {{ isExporting ? $t('tokenList.exporting') : $t('tokenList.confirmExport') }}
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- 刷新选项对话框 -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showRefreshOptions" class="refresh-options-overlay" @click="showRefreshOptions = false">
          <div class="refresh-options-dialog" @click.stop>
            <div class="dialog-header">
              <h3>{{ $t('tokenList.refreshOptions') }}</h3>
              <button class="close-btn" @click="showRefreshOptions = false">×</button>
            </div>
            <div class="dialog-body">
              <p class="dialog-description">{{ $t('tokenList.selectRefreshMode') }}</p>
              <div class="refresh-options">
                <button
                  class="option-btn current-page"
                  @click="executeRefresh('current')"
                >
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zm-5-7h3v2h-3zm-4 0h3v2H5z"/>
                  </svg>
                  <span>{{ $t('tokenList.refreshCurrentPage') }}</span>
                  <p class="option-desc">{{ $t('tokenList.refreshCurrentPageDesc') }}</p>
                </button>
                <button
                  class="option-btn all"
                  @click="executeRefresh('all')"
                >
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 6.5c2.33 0 4.31-1.46 5.11-3.5H6.89c.8 2.04 2.78 3.5 5.11 3.5z"/>
                  </svg>
                  <span>{{ $t('tokenList.refreshAll') }}</span>
                  <p class="option-desc">{{ $t('tokenList.refreshAllDesc') }}</p>
                </button>
              </div>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- 去重模态框 -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showDuplicateModal" class="duplicate-modal-overlay" @click="showDuplicateModal = false">
          <div class="duplicate-modal-content" @click.stop>
            <div class="duplicate-modal-header">
              <h3>🔍 重复Token检测</h3>
              <button class="close-btn" @click="showDuplicateModal = false">×</button>
            </div>

            <div class="duplicate-modal-body">
              <!-- 统计信息卡片 -->
              <div class="duplicate-stats-card">
                <div class="stat-item">
                  <div class="stat-label">总Token数</div>
                  <div class="stat-value">{{ tokens.length }}</div>
                </div>
                <div class="stat-item warning">
                  <div class="stat-label">重复邮箱数</div>
                  <div class="stat-value">{{ duplicateInfo.duplicateEmails.length }}</div>
                </div>
                <div class="stat-item danger">
                  <div class="stat-label">重复Token数</div>
                  <div class="stat-value">{{ duplicateInfo.duplicateCount }}</div>
                </div>
              </div>

              <!-- 重复详情列表 -->
              <div class="duplicate-list">
                <div v-for="duplicate in duplicateInfo.duplicateEmails" :key="duplicate.email" class="duplicate-group">
                  <div class="duplicate-group-header">
                    <span class="email-text">📧 {{ duplicate.email }}</span>
                    <span class="count-badge">重复 {{ duplicate.count }} 次</span>
                  </div>
                  <div class="duplicate-tokens">
                    <div
                      v-for="(tokenId, index) in duplicate.tokenIds"
                      :key="tokenId"
                      :class="['duplicate-token-item', index === 0 ? 'keep' : 'delete']"
                    >
                      <div class="token-info">
                        <span class="token-id">ID: {{ tokenId.substring(0, 8) }}...</span>
                        <span class="token-date">创建于: {{ getTokenDate(tokenId) }}</span>
                      </div>
                      <div :class="['token-action', index === 0 ? 'keep' : 'delete']">
                        {{ index === 0 ? '✅ 保留' : '🗑️ 删除' }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div class="duplicate-modal-footer">
              <button @click="showDuplicateModal = false" class="btn secondary">
                取消
              </button>
              <button
                @click="executeDeduplication"
                class="btn danger"
                :disabled="isDeduplicating"
              >
                {{ isDeduplicating ? '去重中...' : '确认去重' }}
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup>
import { ref, nextTick, onMounted, onUnmounted, computed, readonly, watch } from 'vue'
import { watchDebounced } from '@vueuse/core'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useI18n } from 'vue-i18n'
import { downloadDir } from '@tauri-apps/api/path'
import TokenCard from './TokenCard.vue'
import DatabaseConfig from './DatabaseConfig.vue'
import TokenForm from './TokenForm.vue'

const { t } = useI18n()

// Props - 移除存储状态相关的props，TokenList自主管理
const props = defineProps({
  // 如果将来需要其他props可以在这里添加
})

// 内部状态管理 - TokenList直接管理tokens和存储状态
const tokens = ref([])
const isLoading = ref(false)
const isDatabaseAvailable = ref(false)
const isStorageInitializing = ref(false)

// 初始化就绪标记
const isReady = ref(false)

// 同步状态标记 - 用于防止同步时触发自动保存
const isSyncing = ref(false)
const isLoadingFromSync = ref(false)

// 同步需求标记 - 标识本地有未同步到数据库的更改
const isSyncNeeded = ref(false)

// 存储状态检查定时器
let storageCheckTimer = null

// 事件监听器取消函数
let unlistenTokensUpdated = null

// 排序状态管理
const sortType = ref('time') // 'time' = 按时间排序, 'balance' = 按余额排序
const sortOrder = ref('desc') // 'desc' = 最新优先/余额从多到少, 'asc' = 最旧优先/余额从少到多
const showSortMenu = ref(false) // 排序下拉菜单显示状态

// 搜索状态管理
const searchQuery = ref('')

// 刷新选项对话框状态
const showRefreshOptions = ref(false)
const refreshMode = ref('current') // 'current' = 当前页, 'all' = 全部

// 过滤状态管理
const filterMode = ref('all') // 'all' = 全部, 'normal' = 正常, 'abnormal' = 异常, 'bindcard' = 已绑卡, 'unbindcard' = 未绑卡
const showFilterMenu = ref(false) // 筛选下拉菜单是否显示

// 余额筛选状态管理
const balanceFilterEnabled = ref(false) // 是否启用余额筛选
const balanceFilterMin = ref(null) // 最小余额
const balanceFilterMax = ref(null) // 最大余额
const showBalanceFilterPanel = ref(false) // 余额筛选面板显示状态

// 分页状态管理
const currentPage = ref(1)           // 当前页码
const pageSize = ref(20)             // 每页显示数量(默认 20)
const pageSizeOptions = [10, 20, 50, 100, 200]  // 可选的每页数量

// 高亮状态管理
const highlightedTokenId = ref(null)
let highlightTimer = null

// 多选状态管理
const selectionMode = ref(false) // 是否开启多选模式
const selectedTokenIds = ref(new Set()) // 选中的token ID集合

const DEFAULT_TAG_COLOR = '#f97316'

// 批量删除状态
const showBatchDeleteDialog = ref(false)
const isDeleting = ref(false)

// 导出状态
const showExportDialog = ref(false)
const isExporting = ref(false)
const exportCount = ref('all') // 'all' 或具体数字 10, 20, 50, 100
const exportAndDelete = ref(false) // 导出后是否删除

// 批量导入状态
const showBatchImportDialog = ref(false)
const batchImportTab = ref('session') // 'session' 或 'token'
const importJsonText = ref('')
const isImporting = ref(false)
const importPreview = ref([])
const importErrors = ref([])

// Session 动态输入框状态
const sessionInputs = ref([])
let nextSessionInputId = 1

// Session 批量导入模式状态
const sessionImportMode = ref('single') // 'single' 或 'multi'
const sessionBatchText = ref('')
const parsedSessions = ref([])

// 右键菜单状态
const showContextMenu = ref(false)
const contextMenuPosition = ref({ x: 0, y: 0 })
const contextMenuType = ref('') // 'session' 或 'token'
const customFillCount = ref(1)

// localStorage 配置键名
const STORAGE_KEY_DEFAULT_INPUT_COUNT = 'atm-default-session-input-count'

// UI 配置
const defaultInputCount = ref(3)

// 从 localStorage 加载配置
const loadDefaultInputCount = () => {
  try {
    const stored = localStorage.getItem(STORAGE_KEY_DEFAULT_INPUT_COUNT)
    if (stored) {
      const count = parseInt(stored, 10)
      if (!isNaN(count) && count >= 1 && count <= 20) {
        return count
      }
    }
  } catch (error) {
    console.warn('Failed to load default input count from localStorage:', error)
  }
  return 3 // 默认值
}

// 保存配置到 localStorage
const saveDefaultInputCount = (count) => {
  try {
    localStorage.setItem(STORAGE_KEY_DEFAULT_INPUT_COUNT, count.toString())
    return true
  } catch (error) {
    console.error('Failed to save default input count to localStorage:', error)
    return false
  }
}

// 智能提取email字段的辅助函数
// 支持多种外部格式的email字段命名
const extractEmail = (item) => {
  // 按优先级顺序查找email字段
  const emailFields = [
    'email_note',    // 优先：当前应用标准格式
    'email',         // 次优：常见外部格式
    'emailNote',     // 驼峰格式
    'Email',         // 首字母大写
    'user_email',    // 带前缀
    'userEmail',     // 驼峰带前缀
    'mail'           // 简写
  ]
  
  for (const field of emailFields) {
    const value = item[field]
    // 验证字段存在、类型正确且值有效
    if (value && typeof value === 'string' && value.trim()) {
      return value.trim()
    }
  }
  
  return null
}

// 初始化 Session 输入框
const initializeSessionInputs = (count) => {
  const inputs = []
  for (let i = 1; i <= count; i++) {
    inputs.push({ id: i, value: '' })
  }
  sessionInputs.value = inputs
  nextSessionInputId = count + 1
}

// 填充 Session 模板
const fillSessionTemplate = (count = 1) => {
  const sessions = []
  for (let i = 0; i < count; i++) {
    sessions.push(i === 0 ? 'session1' : '')
  }
  importJsonText.value = JSON.stringify(sessions, null, 2)
  validateImportJson()
}

// 填充 Token 模板
const fillTokenTemplate = (count = 1) => {
  const tokens = []
  for (let i = 0; i < count; i++) {
    tokens.push({
      access_token: i === 0 ? 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...' : '',
      tenant_url: i === 0 ? 'https://example.com' : '',
      email_note: i === 0 ? 'user@example.com' : '',
      portal_url: i === 0 ? 'https://portal.example.com' : ''
    })
  }
  importJsonText.value = JSON.stringify(tokens, null, 2)
  validateImportJson()
}

// 计算可删除的 token 数量
const deletableTokensCount = computed(() => {
  // 与统计逻辑保持一致：SUSPENDED、EXPIRED、INVALID_TOKEN 都是异常状态
  const abnormalStatuses = ['SUSPENDED', 'EXPIRED', 'INVALID_TOKEN']
  return tokens.value.filter(token =>
    abnormalStatuses.includes(token.ban_status)
  ).length
})

// 计算已封禁的 token 数量
const bannedTokensCount = computed(() => {
  return tokens.value.filter(token => token.ban_status === 'SUSPENDED').length
})

// 计算已过期的 token 数量
const expiredTokensCount = computed(() => {
  return tokens.value.filter(token => token.ban_status === 'EXPIRED').length
})

// ==================== 强制推送到数据库相关状态 ====================
const isForcePushing = ref(false)

// ==================== 去重功能相关状态 ====================
// 去重模态框显示状态
const showDuplicateModal = ref(false)
// 去重执行中状态
const isDeduplicating = ref(false)
// 去重信息
const duplicateInfo = ref({
  hasDuplicates: false,      // 是否有重复
  duplicateCount: 0,          // 重复token数量(只计算多余的)
  duplicateEmails: []         // 重复的邮箱列表 [{email, count, tokenIds: []}]
})

// ==================== 批量获取额度相关状态 ====================
// 批量获取额度执行中状态
const isBatchGettingCredits = ref(false)

// 计算需要获取额度的token数量
const tokensNeedingCredits = computed(() => {
  return tokens.value.filter(token => {
    // 没有邮箱或没有portal_info的token
    return !token.email_note || !token.portal_info
  }).length
})

// 统计信息计算属性 - 与 TokenCard 的显示逻辑保持一致
const tokenStats = computed(() => {
  const total = tokens.value.length
  // 只统计显示逻辑中明确定义的异常状态：SUSPENDED、EXPIRED、INVALID_TOKEN
  // 其他状态（ERROR、UNAUTHORIZED 等）在显示时默认为 'active'，所以不统计为异常
  const abnormalStatuses = ['SUSPENDED', 'EXPIRED', 'INVALID_TOKEN']
  const abnormal = tokens.value.filter(token =>
    abnormalStatuses.includes(token.ban_status)
  ).length
  const normal = total - abnormal

  // 按剩余额度统计
  const creditsBelow4000 = tokens.value.filter(token => {
    const credits = token.portal_info?.credits_balance
    return credits !== undefined && credits !== null && credits < 4000
  }).length

  const creditsExact4000 = tokens.value.filter(token => {
    const credits = token.portal_info?.credits_balance
    return credits === 4000
  }).length

  const creditsBetween4001And34000 = tokens.value.filter(token => {
    const credits = token.portal_info?.credits_balance
    return credits !== undefined && credits !== null && credits > 4000 && credits <= 34000
  }).length

  return {
    total,
    normal,
    abnormal,
    creditsBelow4000,
    creditsExact4000,
    creditsBetween4001And34000
  }
})

// 排序后的tokens计算属性
const sortedTokens = computed(() => {
  if (tokens.value.length === 0) return []

  return [...tokens.value].sort((a, b) => {
    if (sortType.value === 'time') {
      // 按时间排序
      const dateA = new Date(a.created_at)
      const dateB = new Date(b.created_at)

      if (sortOrder.value === 'desc') {
        return dateB - dateA // 最新优先
      } else {
        return dateA - dateB // 最旧优先
      }
    } else if (sortType.value === 'balance') {
      // 按余额排序
      const balanceA = a.portal_info?.credits_balance
      const balanceB = b.portal_info?.credits_balance

      // 处理没有余额信息的情况
      const hasBalanceA = balanceA !== undefined && balanceA !== null
      const hasBalanceB = balanceB !== undefined && balanceB !== null

      // 没有余额信息的排在最后
      if (!hasBalanceA && !hasBalanceB) return 0
      if (!hasBalanceA) return 1
      if (!hasBalanceB) return -1

      // 都有余额信息,按数值排序
      if (sortOrder.value === 'desc') {
        return balanceB - balanceA // 余额从多到少
      } else {
        return balanceA - balanceB // 余额从少到多
      }
    }
    return 0
  })
})

// 状态关键词匹配辅助函数 - 支持中英文及常见别名搜索
const matchStatusKeyword = (banStatus, query) => {
  if (!banStatus || !query) return false

  const lowerQuery = query.toLowerCase()

  // 状态关键词映射表（支持中英文及别名）
  const statusKeywords = {
    'ACTIVE': ['active', 'normal', '正常', '激活', '可用'],
    'SUSPENDED': ['suspended', 'banned', 'ban', '封禁', '已封禁', '被封', '禁用'],
    'EXPIRED': ['expired', 'expire', '过期', '已过期', '到期'],
    'INVALID_TOKEN': ['invalid', 'token invalid', '失效', 'token失效', '无效']
  }

  // 获取当前状态的关键词列表
  const keywords = statusKeywords[banStatus] || []

  // 检查是否有任何关键词包含查询词（支持部分匹配）
  return keywords.some(keyword => keyword.includes(lowerQuery))
}

// 筛选模式标签
const filterModeLabel = computed(() => {
  switch (filterMode.value) {
    case 'all': return t('tokenList.filterAll')
    case 'normal': return t('tokenList.filterNormal')
    case 'abnormal': return t('tokenList.filterAbnormal')
    case 'bindcard': return t('tokenList.filterBindCard')
    case 'unbindcard': return t('tokenList.filterUnbindCard')
    default: return t('tokenList.filterAll')
  }
})

// 设置筛选模式
const setFilterMode = (mode) => {
  filterMode.value = mode
  showFilterMenu.value = false
  currentPage.value = 1 // 重置到第一页
}

// 过滤后的tokens计算属性（搜索 + 排序 + 状态过滤 + 余额过滤）
const filteredTokens = computed(() => {
  let result = sortedTokens.value

  // 应用状态过滤 - 与 TokenCard 的显示逻辑保持一致
  switch (filterMode.value) {
    case 'abnormal':
      // 异常状态只包括：SUSPENDED、EXPIRED、INVALID_TOKEN
      result = result.filter(token => {
        const abnormalStatuses = ['SUSPENDED', 'EXPIRED', 'INVALID_TOKEN']
        return abnormalStatuses.includes(token.ban_status)
      })
      break
    case 'normal':
      // 正常状态：不在异常列表中的
      result = result.filter(token => {
        const abnormalStatuses = ['SUSPENDED', 'EXPIRED', 'INVALID_TOKEN']
        return !abnormalStatuses.includes(token.ban_status)
      })
      break
    case 'bindcard':
      // 已绑卡：has_payment_method 为 true
      result = result.filter(token => token.has_payment_method === true)
      break
    case 'unbindcard':
      // 未绑卡：has_payment_method 不为 true（包括 false、null、undefined）
      result = result.filter(token => token.has_payment_method !== true)
      break
    // 'all' 不需要过滤
  }

  // 应用余额范围过滤
  if (balanceFilterEnabled.value && (balanceFilterMin.value !== null || balanceFilterMax.value !== null)) {
    result = result.filter(token => {
      const balance = token.portal_info?.credits_balance

      // 没有余额信息的 token 不符合筛选条件
      if (balance === undefined || balance === null) {
        return false
      }

      // 检查最小值
      if (balanceFilterMin.value !== null && balance < balanceFilterMin.value) {
        return false
      }

      // 检查最大值
      if (balanceFilterMax.value !== null && balance > balanceFilterMax.value) {
        return false
      }

      return true
    })
  }

  // 应用搜索过滤
  if (!searchQuery.value.trim()) {
    return result
  }

  const query = searchQuery.value.toLowerCase().trim()
  return result.filter(token => {
    // 原有字段搜索
    const matchesOriginalFields = (
      token.access_token?.toLowerCase().includes(query) ||
      token.email_note?.toLowerCase().includes(query) ||
      token.auth_session?.toLowerCase().includes(query) ||
      token.tag_name?.toLowerCase().includes(query)
    )

    // 状态搜索（支持中英文关键词）
    const matchesStatus = matchStatusKeyword(token.ban_status, query)

    // 任一匹配即返回
    return matchesOriginalFields || matchesStatus
  })
})

// 总页数
const totalPages = computed(() => {
  return Math.ceil(filteredTokens.value.length / pageSize.value)
})

// 当前页的 tokens
const paginatedTokens = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return filteredTokens.value.slice(start, end)
})

// 分页信息
const paginationInfo = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value + 1
  const end = Math.min(start + pageSize.value - 1, filteredTokens.value.length)
  return {
    start,
    end,
    current: currentPage.value,
    total: filteredTokens.value.length
  }
})

// ==================== 多选相关计算属性 ====================
// 选中的token数量
const selectedCount = computed(() => selectedTokenIds.value.size)

// 选中的tokens
const selectedTokens = computed(() => {
  return tokens.value.filter(token => selectedTokenIds.value.has(token.id))
})

// 当前页是否全选
const isCurrentPageAllSelected = computed(() => {
  if (paginatedTokens.value.length === 0) return false
  return paginatedTokens.value.every(token => selectedTokenIds.value.has(token.id))
})

// 当前页是否部分选中
const isCurrentPagePartiallySelected = computed(() => {
  if (paginatedTokens.value.length === 0) return false
  const selectedInPage = paginatedTokens.value.filter(token => selectedTokenIds.value.has(token.id))
  return selectedInPage.length > 0 && selectedInPage.length < paginatedTokens.value.length
})

// ==================== 分页页码显示逻辑 ====================
const jumpToPageInput = ref('')

// 可见的页码列表
const visiblePages = computed(() => {
  const pages = []
  const total = totalPages.value
  const current = currentPage.value

  // 如果总页数<=7，显示所有页码
  if (total <= 7) {
    for (let i = 1; i <= total; i++) {
      pages.push(i)
    }
    return pages
  }

  // 总页数>7时，显示当前页附近的页码
  let start = Math.max(2, current - 1)
  let end = Math.min(total - 1, current + 1)

  // 确保至少显示3个页码
  if (current <= 3) {
    end = Math.min(5, total - 1)
  } else if (current >= total - 2) {
    start = Math.max(2, total - 4)
  }

  for (let i = start; i <= end; i++) {
    pages.push(i)
  }

  return pages
})

// 是否显示第一页
const showFirstPage = computed(() => {
  return totalPages.value > 1 && !visiblePages.value.includes(1)
})

// 是否显示最后一页
const showLastPage = computed(() => {
  return totalPages.value > 1 && !visiblePages.value.includes(totalPages.value)
})

// 是否显示左侧省略号
const showLeftEllipsis = computed(() => {
  return visiblePages.value.length > 0 && visiblePages.value[0] > 2
})

// 是否显示右侧省略号
const showRightEllipsis = computed(() => {
  return visiblePages.value.length > 0 && visiblePages.value[visiblePages.value.length - 1] < totalPages.value - 1
})

// 跳转到指定页
const goToPage = (page) => {
  if (page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    // 滚动到顶部
    nextTick(() => {
      const container = document.querySelector('.token-grid')
      if (container) {
        container.scrollTop = 0
      }
    })
  }
}

// 处理页码跳转
const handleJumpToPage = () => {
  const page = parseInt(jumpToPageInput.value)
  if (!isNaN(page) && page >= 1 && page <= totalPages.value) {
    currentPage.value = page
    jumpToPageInput.value = ''
  } else {
    window.$notify.warning(`请输入1-${totalPages.value}之间的页码`)
  }
}

// 切换排序方式
const toggleSort = (type = 'time') => {
  // 如果切换到不同的排序类型,重置为降序
  if (sortType.value !== type) {
    sortType.value = type
    sortOrder.value = 'desc'
  } else {
    // 同一类型,切换升序/降序
    sortOrder.value = sortOrder.value === 'desc' ? 'asc' : 'desc'
  }

  // 清空高亮状态，避免排序时重新触发动画
  if (highlightedTokenId.value) {
    highlightedTokenId.value = null
    if (highlightTimer) {
      clearTimeout(highlightTimer)
      highlightTimer = null
    }
  }
}

// 切换排序菜单显示
const toggleSortMenu = () => {
  showSortMenu.value = !showSortMenu.value
}

// 设置排序类型和顺序
const setSortType = (type, order) => {
  sortType.value = type
  sortOrder.value = order
  showSortMenu.value = false

  // 清空高亮状态
  if (highlightedTokenId.value) {
    highlightedTokenId.value = null
    if (highlightTimer) {
      clearTimeout(highlightTimer)
      highlightTimer = null
    }
  }
}

// 应用余额筛选
const applyBalanceFilter = () => {
  // 验证输入
  const min = balanceFilterMin.value
  const max = balanceFilterMax.value

  if (min !== null && max !== null && min > max) {
    window.$notify.warning(t('tokenList.balanceFilterInvalid'))
    return
  }

  balanceFilterEnabled.value = true
  showBalanceFilterPanel.value = false
  // 重置分页到第一页
  currentPage.value = 1
  window.$notify.success(t('tokenList.balanceFilterApplied'))
}

// 清除余额筛选
const clearBalanceFilter = () => {
  balanceFilterEnabled.value = false
  balanceFilterMin.value = null
  balanceFilterMax.value = null
  showBalanceFilterPanel.value = false
  // 重置分页到第一页
  currentPage.value = 1
  window.$notify.success(t('tokenList.balanceFilterCleared'))
}

// 复制筛选出的所有邮箱
const copyFilteredEmails = async () => {
  if (filteredTokens.value.length === 0) {
    window.$notify.warning(t('tokenList.noEmailsToCopy'))
    return
  }

  // 提取所有有邮箱的 token 的邮箱
  const emails = filteredTokens.value
    .filter(token => token.email_note && token.email_note.trim())
    .map(token => token.email_note.trim())

  if (emails.length === 0) {
    window.$notify.warning(t('tokenList.noEmailsFound'))
    return
  }

  // 用换行符连接邮箱
  const emailText = emails.join('\n')

  try {
    // 复制到剪贴板
    await navigator.clipboard.writeText(emailText)
    window.$notify.success(t('tokenList.emailsCopied', { count: emails.length }))
  } catch (error) {
    console.error('Failed to copy emails:', error)
    // 降级方案：使用传统方法
    const textarea = document.createElement('textarea')
    textarea.value = emailText
    document.body.appendChild(textarea)
    textarea.select()
    document.execCommand('copy')
    document.body.removeChild(textarea)
    window.$notify.success(t('tokenList.emailsCopied', { count: emails.length }))
  }
}

// 快速筛选按状态分类的 tokens
const quickFilterByStatus = (statusType) => {
  // 关闭排序菜单
  showSortMenu.value = false

  // 清空余额筛选
  balanceFilterEnabled.value = false
  balanceFilterMin.value = null
  balanceFilterMax.value = null

  // 设置状态筛选
  switch (statusType) {
    case 'all':
      // 显示全部,清空搜索和状态过滤
      filterMode.value = 'all'
      searchQuery.value = ''
      window.$notify.info('已显示全部Token')
      break
    case 'normal':
      // 显示正常状态,清空异常过滤
      filterMode.value = 'all'
      // 使用搜索功能筛选正常状态
      searchQuery.value = '正常'
      window.$notify.info('已筛选正常Token')
      break
    case 'abnormal':
      // 显示异常状态
      filterMode.value = 'abnormal'
      searchQuery.value = ''
      window.$notify.info('已筛选异常Token')
      break
  }

  // 重置到第一页
  currentPage.value = 1
}

// 快速筛选按额度分类的 tokens
const quickFilterByCredits = (filterType) => {
  // 关闭排序菜单
  showSortMenu.value = false

  // 清空状态筛选和搜索
  filterMode.value = 'all'
  searchQuery.value = ''

  // 设置余额筛选范围
  switch (filterType) {
    case 'below4000':
      balanceFilterMin.value = null
      balanceFilterMax.value = 3999
      break
    case 'exact4000':
      balanceFilterMin.value = 4000
      balanceFilterMax.value = 4000
      break
    case 'between4001And34000':
      balanceFilterMin.value = 4001
      balanceFilterMax.value = 34000
      break
  }

  // 启用余额筛选
  balanceFilterEnabled.value = true

  // 重置到第一页
  currentPage.value = 1

  // 显示通知
  window.$notify.info('已应用额度筛选')
}

// 处理模态框内容点击 (关闭排序菜单和筛选菜单)
const handleModalContentClick = (event) => {
  const target = event.target

  // 关闭排序菜单
  if (showSortMenu.value) {
    const sortDropdown = document.querySelector('.sort-dropdown')
    if (sortDropdown && !sortDropdown.contains(target)) {
      showSortMenu.value = false
    }
  }

  // 关闭筛选菜单
  if (showFilterMenu.value) {
    const filterDropdown = document.querySelector('.filter-dropdown')
    if (filterDropdown && !filterDropdown.contains(target)) {
      showFilterMenu.value = false
    }
  }
}

// ==================== 滚动功能 ====================
// 直达顶部
const scrollToTop = () => {
  const container = document.querySelector('.modal-body')
  if (container) {
    container.scrollTo({
      top: 0,
      behavior: 'smooth'
    })
  }
}

// 直达底部
const scrollToBottom = () => {
  const container = document.querySelector('.modal-body')
  if (container) {
    container.scrollTo({
      top: container.scrollHeight,
      behavior: 'smooth'
    })
  }
}

// 上一页
const prevPage = () => {
  if (currentPage.value > 1) {
    goToPage(currentPage.value - 1)
  }
}

// 下一页
const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    goToPage(currentPage.value + 1)
  }
}

// 切换每页数量
const changePageSize = (newSize) => {
  pageSize.value = newSize
  // 保存到 localStorage
  localStorage.setItem('tokenListPageSize', newSize.toString())
  // 重置页码到第 1 页
  currentPage.value = 1
}

// 从 localStorage 加载每页数量
const loadPageSize = () => {
  const saved = localStorage.getItem('tokenListPageSize')
  if (saved) {
    const size = parseInt(saved)
    if (pageSizeOptions.includes(size)) {
      pageSize.value = size
    }
  }
}

// 处理右键菜单
const handleContextMenu = (event, type) => {
  event.preventDefault()
  contextMenuType.value = type
  contextMenuPosition.value = { x: event.clientX, y: event.clientY }
  showContextMenu.value = true
}

// 关闭右键菜单
const closeContextMenu = () => {
  showContextMenu.value = false
}

// 选择填充数量
const selectFillCount = (count) => {
  if (contextMenuType.value === 'session') {
    fillSessionTemplate(count)
  } else if (contextMenuType.value === 'token') {
    fillTokenTemplate(count)
  }
  closeContextMenu()
}

// 从输入框设置默认数量
const setDefaultCountFromInput = () => {
  const count = parseInt(customFillCount.value)

  // 验证范围
  if (isNaN(count) || count < 1 || count > 20) {
    window.$notify.warning(t('tokenList.invalidDefaultCount'))
    return
  }

  // 保存到 localStorage
  if (saveDefaultInputCount(count)) {
    defaultInputCount.value = count
    // 立即重新初始化输入框
    initializeSessionInputs(count)
    window.$notify.success(t('tokenList.defaultCountSaved', { count: count }))
  } else {
    window.$notify.error(t('tokenList.saveDefaultFailed'))
  }

  closeContextMenu()
}

// 使用自定义数量填充（Token Tab）
const fillWithCustomCount = () => {
  const count = parseInt(customFillCount.value)
  if (isNaN(count) || count < 1) {
    window.$notify.warning(t('tokenList.invalidFillCount'))
    return
  }
  if (count > 100) {
    window.$notify.warning(t('tokenList.fillCountTooLarge'))
    return
  }
  selectFillCount(count)
}

// 处理批量删除按钮点击
const handleBatchDelete = () => {
  showBatchDeleteDialog.value = true
}

// 显示批量删除确认对话框
const showBatchDeleteConfirm = () => {
  if (deletableTokensCount.value > 0) {
    showBatchDeleteDialog.value = true
  }
}

// 显示批量导入对话框
const showBatchImportConfirm = () => {
  // 使用配置的默认数量重置 Session 输入框
  initializeSessionInputs(defaultInputCount.value)

  // 重置 Token JSON 输入
  importJsonText.value = '[\n  \n]'
  importPreview.value = []
  importErrors.value = []

  // 重置多行粘贴模式
  sessionImportMode.value = 'single'
  sessionBatchText.value = ''
  parsedSessions.value = []

  // 默认显示 Session Tab
  batchImportTab.value = 'session'

  showBatchImportDialog.value = true
}

// 解析多行 Session 文本
const parseSessionBatch = () => {
  const text = sessionBatchText.value.trim()
  if (!text) {
    parsedSessions.value = []
    return
  }

  // 按行分割，过滤空行和注释行
  const lines = text.split('\n')
    .map(line => line.trim())
    .filter(line => line && !line.startsWith('#'))

  // 验证每一行是否是有效的 Session
  const validSessions = []
  lines.forEach((line, index) => {
    // 移除可能的行号前缀（如 "1. session..." 或 "1: session..."）
    let session = line.replace(/^\d+[\.\:\s]+/, '').trim()

    // 基本验证
    if (session.length >= 10) {
      validSessions.push(session)
    }
  })

  parsedSessions.value = validSessions
}

// 清空多行粘贴内容
const clearSessionBatch = () => {
  sessionBatchText.value = ''
  parsedSessions.value = []
}

// 脱敏显示 Session
const maskSession = (session) => {
  if (!session || session.length < 10) return session
  return session.substring(0, 4) + '...' + session.substring(session.length - 1)
}

// 获取当前模式下的 Session 数量
const getSessionCountForImport = () => {
  if (sessionImportMode.value === 'single') {
    return validSessionCount.value
  } else {
    return parsedSessions.value.length
  }
}

// Session 动态输入框方法
const addSessionInput = () => {
  sessionInputs.value.push({
    id: nextSessionInputId++,
    value: ''
  })
}

const removeSessionInput = (id) => {
  if (sessionInputs.value.length <= 1) {
    window.$notify.warning(t('tokenList.atLeastOneInput'))
    return
  }
  sessionInputs.value = sessionInputs.value.filter(input => input.id !== id)
}

// 获取有效的 Session 输入数量
const validSessionCount = computed(() => {
  return sessionInputs.value.filter(input => input.value.trim()).length
})

// 并发控制函数：限制同时执行的 Promise 数量
const executeWithConcurrency = async (tasks, concurrency = 5) => {
  const results = []
  const executing = []

  for (let i = 0; i < tasks.length; i++) {
    const promise = Promise.resolve()
      .then(() => tasks[i]())
      .then(
        result => { results[i] = { success: true, result } },
        error => { results[i] = { success: false, error } }
      )
      .catch(error => {
        // 确保即使 Promise 出错也能被记录
        results[i] = { success: false, error }
      })

    executing.push(promise)

    if (executing.length >= concurrency) {
      await Promise.race(executing)
      const completedIndex = executing.findIndex(p => p === promise)
      if (completedIndex !== -1) {
        executing.splice(completedIndex, 1)
      }
    }
  }

  await Promise.all(executing)

  // 过滤掉 undefined 的结果，确保返回有效数据
  return results.filter(r => r !== undefined)
}

// 从 Session 动态输入框执行批量导入
const executeBatchImportFromSessionInputs = async () => {
  // 根据模式获取 Session 列表
  let validSessions = []

  if (sessionImportMode.value === 'single') {
    // 单行模式：从输入框获取
    validSessions = sessionInputs.value
      .map(input => input.value.trim())
      .filter(value => value.length > 0)
  } else {
    // 多行模式：从已解析的 Session 列表获取
    validSessions = parsedSessions.value
  }

  if (validSessions.length === 0) {
    window.$notify.warning(t('tokenList.noValidSessions'))
    return
  }

  isImporting.value = true

  try {
    let successCount = 0
    let skippedCount = 0
    let sessionExtractionErrors = []
    let duplicateIds = []

    // 创建任务列表（并发执行）
    const tasks = validSessions.map((session, index) => async () => {
      try {
        // 调用后端 API 从 session 提取 token
        const result = await invoke('add_token_from_session', {
          session: session
        })

        // 提取成功,添加 token
        const tokenData = {
          tenantUrl: result.tenant_url,
          accessToken: result.access_token,
          portalUrl: null,
          emailNote: result.email || null,
          authSession: session,
          suspensions: null,
          creditsBalance: null,  // Session 导入不再获取余额
          expiryDate: null,  // Session 导入不再获取过期时间
          banStatus: 'ACTIVE'
        }

        const addResult = addToken(tokenData)
        return {
          success: true,
          addResult,
          index,
          email: result.email || null,  // 保存邮箱信息
          sessionNumber: index + 1  // 保存 session 号（从 1 开始）
        }
      } catch (error) {
        console.error('Failed to extract token from session:', error)
        return {
          success: false,
          error: error.toString(),
          index,
          sessionNumber: index + 1,  // 保存 session 号（从 1 开始）
          email: null
        }
      }
    })

    // 并发执行所有任务（最多 5 个同时进行）
    const results = await executeWithConcurrency(tasks, 5)

    // 处理结果
    for (const result of results) {
      if (!result || !result.success) {
        // 处理失败的结果
        if (result && result.error) {
          const errorInfo = {
            sessionNumber: result.error.sessionNumber || result.error.index + 1,
            error: result.error.error || result.error,
            email: result.error.email || null
          }
          sessionExtractionErrors.push(errorInfo)
          // 单条失败提示
          window.$notify.error(`❌ Session ${errorInfo.sessionNumber} 导入失败: ${errorInfo.error}`)
        }
        skippedCount++
        continue
      }

      // 处理成功的结果
      const taskResult = result.result
      if (taskResult && taskResult.addResult) {
        const { addResult, email, sessionNumber } = taskResult
        if (addResult.success) {
          successCount++
          // 单条成功提示
          const emailInfo = email ? ` (${email})` : ''
          window.$notify.success(`✅ Session ${sessionNumber}${emailInfo} 导入成功`)
        } else if (addResult.duplicateId) {
          duplicateIds.push(addResult.duplicateId)
          skippedCount++
          // 重复提示
          window.$notify.warning(`⚠️ Session ${sessionNumber} 重复，已跳过`)
        } else {
          skippedCount++
        }
      } else {
        skippedCount++
      }
    }

    // 关闭对话框
    showBatchImportDialog.value = false

    // 显示最终汇总
    const totalCount = validSessions.length
    const failedCount = sessionExtractionErrors.length + (skippedCount - duplicateIds.length)

    if (totalCount === successCount) {
      // 全部成功
      window.$notify.success(`🎉 已全部完成！导入成功 ${successCount} 个`)
    } else {
      // 有失败的
      const message = `📊 批量导入完成！\n✅ 导入成功: ${successCount} 个\n❌ 导入失败: ${failedCount} 个`
      if (failedCount > 0) {
        window.$notify.warning(message)
      } else {
        window.$notify.success(message)
      }
    }
  } catch (error) {
    window.$notify.error(`${t('messages.batchImportFailed')}: ${error}`)
  } finally {
    isImporting.value = false
  }
}

// 验证并解析导入的 JSON
const validateImportJson = () => {
  importErrors.value = []
  importPreview.value = []

  if (!importJsonText.value.trim()) {
    importErrors.value.push(t('tokenList.importJsonEmpty'))
    return false
  }

  try {
    const parsed = JSON.parse(importJsonText.value)

    if (!Array.isArray(parsed)) {
      importErrors.value.push(t('tokenList.importJsonNotArray'))
      return false
    }

    if (parsed.length === 0) {
      importErrors.value.push(t('tokenList.importJsonEmptyArray'))
      return false
    }

    // 检测数组类型: 字符串数组(Session) 或 对象数组(Token)
    const firstItem = parsed[0]
    const isSessionArray = typeof firstItem === 'string'
    const isTokenArray = typeof firstItem === 'object' && firstItem !== null

    if (!isSessionArray && !isTokenArray) {
      importErrors.value.push('数组元素必须是字符串(Session)或对象(Token)')
      return false
    }

    // 验证 Session 数组
    if (isSessionArray) {
      const validSessions = []
      parsed.forEach((item, index) => {
        if (typeof item !== 'string') {
          importErrors.value.push(`[${index + 1}] 必须是字符串`)
        } else if (!item.trim()) {
          importErrors.value.push(`[${index + 1}] Session 不能为空`)
        } else if (item.length < 10) {
          importErrors.value.push(`[${index + 1}] ${t('tokenList.invalidSession')}`)
        } else {
          validSessions.push({ auth_session: item })
        }
      })
      importPreview.value = validSessions
      return validSessions.length > 0
    }

    // 验证 Token 对象数组
    if (isTokenArray) {
      const validTokens = []
      parsed.forEach((item, index) => {
        const errors = []

        if (typeof item !== 'object' || item === null) {
          importErrors.value.push(`[${index + 1}] 必须是对象`)
          return
        }

        // 验证必需字段
        if (!item.access_token || typeof item.access_token !== 'string' || !item.access_token.trim()) {
          errors.push(`[${index + 1}] ${t('tokenList.missingAccessToken')}`)
        }

        if (!item.tenant_url || typeof item.tenant_url !== 'string' || !item.tenant_url.trim()) {
          errors.push(`[${index + 1}] ${t('tokenList.missingTenantUrl')}`)
        }

        // 验证 URL 格式
        if (item.tenant_url) {
          try {
            new URL(item.tenant_url)
          } catch {
            errors.push(`[${index + 1}] ${t('tokenList.invalidTenantUrl')}`)
          }
        }

        if (item.portal_url) {
          try {
            new URL(item.portal_url)
          } catch {
            errors.push(`[${index + 1}] ${t('tokenList.invalidPortalUrl')}`)
          }
        }

        if (errors.length > 0) {
          importErrors.value.push(...errors)
        } else {
          validTokens.push(item)
        }
      })
      importPreview.value = validTokens
      return validTokens.length > 0
    }

    return false
  } catch (error) {
    importErrors.value.push(`${t('tokenList.importJsonParseError')}: ${error.message}`)
    return false
  }
}

// 执行批量导入
const executeBatchImport = async () => {
  // 如果是 Session Tab,从动态输入框导入
  if (batchImportTab.value === 'session') {
    await executeBatchImportFromSessionInputs()
    return
  }

  // Token Tab: 使用原有的 JSON 导入逻辑
  if (!validateImportJson()) {
    return
  }

  isImporting.value = true

  try {
    let successCount = 0
    let skippedCount = 0
    let sessionExtractionErrors = []
    let duplicateIds = []  // 收集重复的 token ID

    // 检测导入模式: Session 数组 或 Token 对象数组
    const firstItem = importPreview.value[0]
    const isSessionMode = firstItem.auth_session && !firstItem.access_token

    if (isSessionMode) {
      // Session 模式: 并发提取所有 session
      const tasks = importPreview.value.map((item, index) => async () => {
        try {
          // 调用后端 API 从 session 提取 token
          const result = await invoke('add_token_from_session', {
            session: item.auth_session
          })

          // 提取成功,添加 token
          const tokenData = {
            tenantUrl: result.tenant_url,
            accessToken: result.access_token,
            portalUrl: null,
            emailNote: result.email || null,
            authSession: item.auth_session,
            suspensions: null,
            creditsBalance: null,  // Session 导入不再获取余额
            expiryDate: null  // Session 导入不再获取过期时间
          }

          const addResult = addToken(tokenData)
          return {
            success: true,
            addResult,
            index,
            email: result.email || null,
            sessionNumber: index + 1
          }
        } catch (error) {
          console.error('Failed to extract token from session:', error)
          return {
            success: false,
            error: error.toString(),
            index,
            sessionNumber: index + 1,
            email: null
          }
        }
      })

      // 并发执行所有任务（最多 5 个同时进行）
      const results = await executeWithConcurrency(tasks, 5)

      // 处理结果
      for (const result of results) {
        if (!result || !result.success) {
          // 处理失败的结果
          if (result && result.error) {
            const errorInfo = {
              sessionNumber: result.error.sessionNumber || result.error.index + 1,
              error: result.error.error || result.error,
              email: result.error.email || null
            }
            sessionExtractionErrors.push(errorInfo)
            // 单条失败提示
            window.$notify.error(`❌ Session ${errorInfo.sessionNumber} 导入失败: ${errorInfo.error}`)
          }
          skippedCount++
          continue
        }

        // 处理成功的结果
        const taskResult = result.result
        if (taskResult && taskResult.addResult) {
          const { addResult, email, sessionNumber } = taskResult
          if (addResult.success) {
            successCount++
            // 单条成功提示
            const emailInfo = email ? ` (${email})` : ''
            window.$notify.success(`✅ Session ${sessionNumber}${emailInfo} 导入成功`)
          } else if (addResult.duplicateId) {
            duplicateIds.push(addResult.duplicateId)
            skippedCount++
            // 重复提示
            window.$notify.warning(`⚠️ Session ${sessionNumber} 重复，已跳过`)
          } else {
            skippedCount++
          }
        } else {
          skippedCount++
        }
      }
    } else {
      // Token 模式: 直接添加所有 token（同步操作，无需并发）
      importPreview.value.forEach((item, index) => {
        const tokenData = {
          tenantUrl: item.tenant_url,
          accessToken: item.access_token,
          portalUrl: item.portal_url || null,
          emailNote: extractEmail(item),
          tagName: item.tag_name || null,
          tagColor: item.tag_color || null,
          authSession: null,
          suspensions: item.suspensions || null
        }

        const result = addToken(tokenData)
        const tokenNumber = index + 1
        const emailInfo = tokenData.emailNote ? ` (${tokenData.emailNote})` : ''

        if (result.success) {
          successCount++
          // 单条成功提示
          window.$notify.success(`✅ Token ${tokenNumber}${emailInfo} 导入成功`)
        } else if (result.duplicateId) {
          duplicateIds.push(result.duplicateId)
          skippedCount++
          // 重复提示
          window.$notify.warning(`⚠️ Token ${tokenNumber}${emailInfo} 重复，已跳过`)
        } else {
          skippedCount++
        }
      })
    }

    // 关闭对话框
    showBatchImportDialog.value = false

    // 显示最终汇总
    const totalCount = importPreview.value.length
    const failedCount = sessionExtractionErrors.length + (skippedCount - duplicateIds.length)

    if (totalCount === successCount) {
      // 全部成功
      window.$notify.success(`🎉 已全部完成！导入成功 ${successCount} 个`)
    } else {
      // 有失败的
      const message = `📊 批量导入完成！\n✅ 导入成功: ${successCount} 个\n❌ 导入失败: ${failedCount} 个`
      if (failedCount > 0) {
        window.$notify.warning(message)
      } else {
        window.$notify.success(message)
      }
    }
  } catch (error) {
    window.$notify.error(`${t('messages.batchImportFailed')}: ${error}`)
  } finally {
    isImporting.value = false
  }
}

// 执行批量删除
const executeBatchDelete = async () => {
  isDeleting.value = true

  try {
    // 获取要删除的 tokens
    const tokensToDelete = tokens.value.filter(token =>
      token.ban_status === 'SUSPENDED' || token.ban_status === 'EXPIRED'
    )

    // 并行删除所有 tokens
    const deletePromises = tokensToDelete.map(token =>
      invoke('delete_token', { tokenId: token.id })
        .then(() => {
          // 删除成功,从本地列表移除
          const index = tokens.value.findIndex(t => t.id === token.id)
          if (index !== -1) {
            tokens.value.splice(index, 1)
          }
          return { success: true, id: token.id }
        })
        .catch(error => {
          console.error(`Failed to delete token ${token.id}:`, error)
          return { success: false, id: token.id, error }
        })
    )

    // 等待所有删除操作完成
    const results = await Promise.allSettled(deletePromises)

    // 统计成功和失败的数量
    const successCount = results.filter(r =>
      r.status === 'fulfilled' && r.value.success
    ).length
    const failedCount = tokensToDelete.length - successCount

    // 关闭对话框
    showBatchDeleteDialog.value = false

    // 显示结果消息
    if (failedCount === 0) {
      console.log(`Successfully deleted ${successCount} tokens`)
    } else {
      console.warn(`Deleted ${successCount} tokens, ${failedCount} failed`)
    }
  } catch (error) {
    console.error('Batch delete failed:', error)
  } finally {
    isDeleting.value = false
  }
}

// 打开导出对话框
const handleExportTokens = () => {
  if (filteredTokens.value.length === 0) {
    window.$notify.warning(t('tokenList.noTokensToExport'))
    return
  }

  // 重置导出选项
  exportCount.value = 'all'
  exportAndDelete.value = false
  showExportDialog.value = true
}

// 获取导出数量
const getExportCount = () => {
  if (exportCount.value === 'all') {
    return filteredTokens.value.length
  }
  return Math.min(exportCount.value, filteredTokens.value.length)
}

// 执行导出
const executeExport = async () => {
  isExporting.value = true

  try {
    // 获取要导出的 tokens
    const count = getExportCount()
    const tokensToExport = filteredTokens.value.slice(0, count)

    if (tokensToExport.length === 0) {
      window.$notify.warning(t('tokenList.noTokensToExport'))
      return
    }

    // 构建导出数据 - 导出所有字段
    const exportData = tokensToExport.map(token => ({
      id: token.id,
      tenant_url: token.tenant_url,
      access_token: token.access_token,
      created_at: token.created_at,
      updated_at: token.updated_at,
      portal_url: token.portal_url,
      email_note: token.email_note,
      tag_name: token.tag_name,
      tag_color: token.tag_color,
      ban_status: token.ban_status,
      portal_info: token.portal_info,
      auth_session: token.auth_session,
      suspensions: token.suspensions,
      balance_color_mode: token.balance_color_mode,
      skip_check: token.skip_check
    }))

    // 生成 JSON 字符串
    const jsonString = JSON.stringify(exportData, null, 2)

    // 生成文件名：tokens_YYYY-MM-DD_HH-mm-ss.json
    const now = new Date()
    const dateStr = now.toISOString().split('T')[0]
    const timeStr = now.toTimeString().split(' ')[0].replace(/:/g, '-')
    const fileName = `tokens_${dateStr}_${timeStr}.json`

    try {
      // 获取下载文件夹路径
      const downloadPath = await downloadDir()
      const filePath = `${downloadPath}${fileName}`

      // 使用 Tauri fs 插件保存文件
      await writeTextFile(fileName, jsonString, { dir: BaseDirectory.Download })

      // 显示成功消息，包含完整路径
      window.$notify.success(
        `${t('tokenList.exportSuccess', { count: exportData.length })}\n${t('tokenList.downloadedTo')}: ${filePath}`
      )
    } catch (error) {
      console.error('Export failed:', error)
      // 降级到浏览器下载
      const blob = new Blob([jsonString], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = fileName

      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)

      URL.revokeObjectURL(url)

      window.$notify.success(
        `${t('tokenList.exportSuccess', { count: exportData.length })}\n${t('tokenList.downloadedTo')}: ${fileName}`
      )
    }

    // 如果选择了导出后删除
    if (exportAndDelete.value) {
      // 关闭导出对话框
      showExportDialog.value = false

      // 执行删除操作
      await executeExportAndDelete(tokensToExport)
    } else {
      // 关闭导出对话框
      showExportDialog.value = false
    }
  } catch (error) {
    console.error('Export failed:', error)
    window.$notify.error(`${t('tokenList.exportFailed')}: ${error}`)
  } finally {
    isExporting.value = false
  }
}

// 执行导出后删除
const executeExportAndDelete = async (tokensToDelete) => {
  try {
    // 并行删除所有 tokens
    const deletePromises = tokensToDelete.map(token =>
      invoke('delete_token', { tokenId: token.id })
        .then(() => {
          // 删除成功,从本地列表移除
          const index = tokens.value.findIndex(t => t.id === token.id)
          if (index !== -1) {
            tokens.value.splice(index, 1)
          }
          return { success: true, id: token.id }
        })
        .catch(error => {
          console.error(`Failed to delete token ${token.id}:`, error)
          return { success: false, id: token.id, error }
        })
    )

    // 等待所有删除操作完成
    const results = await Promise.allSettled(deletePromises)

    // 统计成功和失败的数量
    const successCount = results.filter(r =>
      r.status === 'fulfilled' && r.value.success
    ).length
    const failedCount = tokensToDelete.length - successCount

    // 显示结果消息
    if (failedCount === 0) {
      window.$notify.success(t('tokenList.exportAndDeleteSuccess', { count: successCount }))
    } else {
      window.$notify.warning(
        t('tokenList.exportAndDeletePartial', { success: successCount, failed: failedCount })
      )
    }
  } catch (error) {
    console.error('Export and delete failed:', error)
    window.$notify.error(`${t('tokenList.exportAndDeleteFailed')}: ${error}`)
  }
}

const emit = defineEmits(['close'])

// Additional state for new components
const showDatabaseConfig = ref(false)
const isSaving = ref(false)
const isRefreshing = ref(false)

// TokenForm state management
const showTokenFormModal = ref(false)
const editingToken = ref(null)

// Token card refs for accessing child methods
const tokenCardRefs = ref({})

// Computed properties for storage status display
const storageStatusText = computed(() => {
  if (isStorageInitializing.value) {
    return t('storage.initializing')
  }
  if (isDatabaseAvailable.value) {
    return isSyncNeeded.value
      ? `${t('storage.dualStorage')}-${t('storage.notSynced')}`
      : t('storage.dualStorage')
  }
  return t('storage.localStorage')
})

const storageStatusClass = computed(() => {
  // 如果正在初始化，显示加载样式
  if (isStorageInitializing.value) {
    return 'initializing'
  }
  // 如果是双向存储且未同步，显示警告样式
  if (isDatabaseAvailable.value && isSyncNeeded.value) {
    return 'unsaved'
  }
  return 'saved'
})



// 存储状态管理方法
const getStorageStatus = async () => {
  try {
    const status = await invoke('get_storage_status')

    // 检查是否正在初始化
    if (status?.is_initializing) {
      isStorageInitializing.value = true
      isDatabaseAvailable.value = false

      // 启动定时器，每 500ms 检查一次
      if (!storageCheckTimer) {
        storageCheckTimer = setInterval(async () => {
          await getStorageStatus()
        }, 500)
      }
    } else {
      // 初始化完成
      isStorageInitializing.value = false
      isDatabaseAvailable.value = status?.is_database_available || false

      // 停止定时器
      if (storageCheckTimer) {
        clearInterval(storageCheckTimer)
        storageCheckTimer = null
      }
    }
  } catch (error) {
    console.error('Failed to get storage status:', error)
    isDatabaseAvailable.value = false
    isStorageInitializing.value = false

    // 停止定时器
    if (storageCheckTimer) {
      clearInterval(storageCheckTimer)
      storageCheckTimer = null
    }
  }
}

// 初始化就绪等待方法
const waitUntilReady = async () => {
  if (isReady.value && !isLoading.value) return
  await new Promise((resolve) => {
    const stop = watch([isReady, isLoading], ([ready, loading]) => {
      if (ready && !loading) {
        stop()
        resolve()
      }
    })
  })
}

// 设置ref的函数
const setTokenCardRef = (el, tokenId) => {
  if (el) {
    tokenCardRefs.value[tokenId] = el
  } else {
    // 当组件被移除时，清理引用
    delete tokenCardRefs.value[tokenId]
  }
}

// 处理 Token 更新事件
const handleTokenUpdated = () => {
  // Token 更新时不再设置未保存状态，关闭时会自动保存
}

// 深度比对两个对象是否相等
const isEqual = (obj1, obj2) => {
  if (obj1 === obj2) return true
  if (obj1 == null || obj2 == null) return false
  if (typeof obj1 !== 'object' || typeof obj2 !== 'object') return obj1 === obj2

  const keys1 = Object.keys(obj1)
  const keys2 = Object.keys(obj2)

  if (keys1.length !== keys2.length) return false

  for (const key of keys1) {
    if (!keys2.includes(key)) return false
    if (!isEqual(obj1[key], obj2[key])) return false
  }

  return true
}

// 检查所有Token的账号状态
const checkAllAccountStatus = async () => {
  if (tokens.value.length === 0) {
    return { success: true, hasChanges: false, message: t('messages.noTokensToCheck') }
  }

  try {
    // 准备批量检测的数据，过滤掉标记为跳过检测的账号
    const tokensToCheck = tokens.value.filter(token => !token.skip_check)

    const tokenInfos = tokensToCheck.map(token => ({
      id: token.id,
      access_token: token.access_token,
      tenant_url: token.tenant_url,
      portal_url: token.portal_url || null,
      auth_session: token.auth_session || null,
      email_note: token.email_note || null
    }))

    // 单次批量API调用
    const results = await invoke('batch_check_tokens_status', {
      tokens: tokenInfos
    })


    // 批量更新tokens状态，返回是否有变化
    const hasChanges = updateTokensFromResults(results)

    return { success: true, hasChanges }

  } catch (error) {
    console.error('Batch check error:', error)
    return {
      success: false,
      hasChanges: false,
      message: `${t('messages.accountStatusCheckError')}: ${error}`
    }
  }
}

// 根据批量检测结果更新tokens状态
const updateTokensFromResults = (results) => {
  let anyChanges = false

  results.forEach(result => {
    const token = tokens.value.find(t => t.id === result.token_id)
    if (token) {
      const statusResult = result.status_result
      let hasChanges = false

      // 比对并更新 access_token
      if (token.access_token !== result.access_token) {
        token.access_token = result.access_token
        hasChanges = true
      }

      // 比对并更新 tenant_url
      if (token.tenant_url !== result.tenant_url) {
        token.tenant_url = result.tenant_url
        hasChanges = true
      }

      // 比对并更新 ban_status
      if (token.ban_status !== statusResult.status) {
        token.ban_status = statusResult.status
        hasChanges = true
      }

      // 自动禁用封禁或过期的账号检测
      if ((statusResult.status === 'SUSPENDED' || statusResult.status === 'EXPIRED') && !token.skip_check) {
        token.skip_check = true
        hasChanges = true
        // 显示通知
        const autoDisableMsg = statusResult.status === 'SUSPENDED'
          ? t('messages.autoDisabledBanned')
          : t('messages.autoDisabledExpired')
        window.$notify.info(autoDisableMsg)
      }

      // 比对并更新 suspensions 信息（如果有）
      if (result.suspensions) {
        if (!isEqual(token.suspensions, result.suspensions)) {
          token.suspensions = result.suspensions
          hasChanges = true
          console.log(`Updated suspensions for token ${token.id}:`, result.suspensions)
        }
      }

      // 比对并更新 Portal 信息（如果有）
      if (result.portal_info) {
        const newPortalInfo = {
          credits_balance: result.portal_info.credits_balance,
          expiry_date: result.portal_info.expiry_date
        }

        if (!isEqual(token.portal_info, newPortalInfo)) {
          token.portal_info = newPortalInfo
          hasChanges = true
          console.log(`Updated token ${token.id} portal info:`, token.portal_info)
        }
      } else if (result.portal_error) {
        console.warn(`Failed to get portal info for token ${token.id}:`, result.portal_error)
      }

      // 比对并更新 email_note（如果有）
      if (result.email_note && token.email_note !== result.email_note) {
        token.email_note = result.email_note
        hasChanges = true
      }

      // 只有在有实际变化时才更新时间戳
      if (hasChanges) {
        token.updated_at = new Date().toISOString()
        anyChanges = true
      }
    }
  })

  return anyChanges
}

const loadTokens = async (showSuccessMessage = false) => {
  isLoading.value = true
  try {
    const jsonString = await invoke('load_tokens_json')
    const parsedTokens = JSON.parse(jsonString)

    // 确保是数组
    if (Array.isArray(parsedTokens)) {
      // 使用展开运算符创建新数组，确保触发响应式更新
      tokens.value = [...parsedTokens]
    } else {
      tokens.value = []
    }

    // 加载完成后检测重复（使用nextTick确保DOM更新）
    await nextTick()
    detectDuplicates()

    if (showSuccessMessage) {
      window.$notify.success(t('messages.tokenLoadSuccess'))
    }
  } catch (error) {
    window.$notify.error(`${t('messages.tokenLoadFailed')}: ${error}`)
    tokens.value = []
  } finally {
    isLoading.value = false
  }
}

const saveTokens = async (showSuccessMessage = false) => {
  try {
    const jsonString = JSON.stringify(tokens.value, null, 2)
    await invoke('save_tokens_json', { jsonString })
    if (showSuccessMessage) {
      window.$notify.success(t('messages.tokenSaved'))
    }
  } catch (error) {
    window.$notify.error(`${t('messages.tokenSaveFailed')}: ${error}`)
    throw error
  }
}

// ==================== 去重功能函数 ====================
// 检测重复（邮箱 + Session）
const detectDuplicates = () => {
  console.log('🔍 开始检测重复，当前token数量:', tokens.value.length)
  const emailMap = new Map()
  const sessionMap = new Map()

  // 遍历所有token，按邮箱和Session分组
  tokens.value.forEach(token => {
    // 按邮箱分组
    if (token.email_note && token.email_note.trim()) {
      const email = token.email_note.trim().toLowerCase()
      if (!emailMap.has(email)) {
        emailMap.set(email, [])
      }
      emailMap.get(email).push(token.id)
    }

    // 按Session分组
    if (token.session && token.session.trim()) {
      const session = token.session.trim()
      if (!sessionMap.has(session)) {
        sessionMap.set(session, [])
      }
      sessionMap.get(session).push(token.id)
    }
  })

  console.log('📧 邮箱分组完成，共有邮箱数:', emailMap.size)
  console.log('🔑 Session分组完成，共有Session数:', sessionMap.size)

  // 找出重复的邮箱
  const duplicates = []
  let totalDuplicateCount = 0

  emailMap.forEach((tokenIds, email) => {
    if (tokenIds.length > 1) {
      duplicates.push({
        email: `📧 ${email}`,
        count: tokenIds.length,
        tokenIds
      })
      totalDuplicateCount += tokenIds.length - 1
      console.log(`⚠️ 发现重复邮箱: ${email}, 重复次数: ${tokenIds.length}`)
    }
  })

  // 找出重复的Session
  sessionMap.forEach((tokenIds, session) => {
    if (tokenIds.length > 1) {
      duplicates.push({
        email: `🔑 Session: ${session.substring(0, 20)}...`,
        count: tokenIds.length,
        tokenIds
      })
      totalDuplicateCount += tokenIds.length - 1
      console.log(`⚠️ 发现重复Session: ${session.substring(0, 20)}..., 重复次数: ${tokenIds.length}`)
    }
  })

  duplicateInfo.value = {
    hasDuplicates: duplicates.length > 0,
    duplicateCount: totalDuplicateCount,
    duplicateEmails: duplicates
  }

  console.log('✅ 检测完成，重复项数:', duplicates.length, '重复token数:', totalDuplicateCount)

  if (duplicates.length > 0) {
    window.$notify.success(`🔍 检测完成！发现 ${duplicates.length} 个重复项，共 ${totalDuplicateCount} 条重复token`)
    // 自动打开详情模态框
    showDuplicateModal.value = true
  } else {
    window.$notify.info('✅ 未发现重复的token')
  }
}

// 获取token的创建日期
const getTokenDate = (tokenId) => {
  const token = tokens.value.find(t => t.id === tokenId)
  if (token && token.created_at) {
    return new Date(token.created_at).toLocaleDateString('zh-CN')
  }
  return '未知'
}

// ==================== 批量获取额度功能 ====================
// 批量获取额度
const batchGetCredits = async () => {
  if (isBatchGettingCredits.value) return

  // 筛选需要获取额度的token
  const tokensToRefresh = tokens.value.filter(token => {
    return !token.email_note || !token.portal_info
  })

  if (tokensToRefresh.length === 0) {
    window.$notify.info('没有需要获取额度的token')
    return
  }

  isBatchGettingCredits.value = true

  try {
    window.$notify.info(`开始批量获取 ${tokensToRefresh.length} 个token的额度...`)

    let successCount = 0
    let failedTokens = []

    // 为每个需要刷新的token创建刷新任务
    const refreshTasks = tokensToRefresh.map((token, index) => async () => {
      try {
        // 找到对应的token在当前页面中的索引
        const tokenIndex = tokens.value.findIndex(t => t.id === token.id)
        if (tokenIndex === -1) {
          throw new Error('Token not found')
        }

        // 触发刷新 - 直接调用后端接口
        const result = await invoke('batch_check_tokens_status', {
          tokens: [{
            id: token.id,
            access_token: token.access_token,
            tenant_url: token.tenant_url,
            portal_url: token.portal_url || null,
            auth_session: token.auth_session || null,
            email_note: token.email_note || null
          }]
        })

        if (result && result.length > 0) {
          const checkResult = result[0]
          const targetToken = tokens.value[tokenIndex]
          const tokenNumber = index + 1

          // 🔥 创建新对象，强制触发Vue响应式更新
          const updatedToken = { ...targetToken }

          // 更新所有返回的信息
          // 1. 更新access_token和tenant_url（可能被刷新）
          if (checkResult.access_token) {
            updatedToken.access_token = checkResult.access_token
          }
          if (checkResult.tenant_url) {
            updatedToken.tenant_url = checkResult.tenant_url
          }

          // 2. 更新邮箱信息
          if (checkResult.email_note) {
            updatedToken.email_note = checkResult.email_note
          }

          // 3. 更新封禁信息
          if (checkResult.suspensions !== undefined) {
            updatedToken.suspensions = checkResult.suspensions
          }

          // 4. 更新账号状态
          if (checkResult.status_result) {
            // 使用后端返回的具体状态值，而不是简化的 'BANNED'
            updatedToken.ban_status = checkResult.status_result.status || 'ACTIVE'
          }

          const emailInfo = updatedToken.email_note ? ` (${updatedToken.email_note})` : ''

          // 5. 更新portal_info（额度信息）
          if (checkResult.portal_info) {
            updatedToken.portal_info = checkResult.portal_info
            successCount++
            window.$notify.success(`✅ Token ${tokenNumber}${emailInfo} 获取额度成功`)
          } else {
            const reason = checkResult.portal_error || '未获取到额度信息'
            failedTokens.push({
              index: tokenNumber,
              email: updatedToken.email_note || '未知',
              id: updatedToken.id,
              reason
            })
            window.$notify.warning(`⚠️ Token ${tokenNumber}${emailInfo} 获取失败: ${reason}`)
          }

          // 6. 更新绑卡链接
          if (checkResult.payment_method_link) {
            updatedToken.payment_method_link = checkResult.payment_method_link
            console.log(`✅ Token ${tokenNumber}${emailInfo} 获取绑卡链接成功`)
          }

          // 🔥 更新 updated_at 时间戳，确保同步时使用最新数据
          updatedToken.updated_at = new Date().toISOString()

          // 🔥 使用数组替换方式，强制触发Vue响应式更新
          tokens.value[tokenIndex] = updatedToken
        }

        return { success: true, tokenId: token.id }
      } catch (error) {
        const emailInfo = token.email_note || '未知'
        failedTokens.push({
          index: index + 1,
          email: emailInfo,
          id: token.id,
          reason: error.toString()
        })
        window.$notify.error(`❌ Token ${index + 1} (${emailInfo}) 获取失败: ${error}`)
        return { success: false, tokenId: token.id, error }
      }
    })

    // 并发执行（最多5个同时进行）
    await executeWithConcurrency(refreshTasks, 5)

    // 保存更新到本地JSON - 等待之前的保存完成
    console.log('📝 准备保存到本地JSON，当前tokens数量:', tokens.value.length)
    let waitCount = 0
    const maxWait = 50 // 最多等待5秒
    while (isSaving.value && waitCount < maxWait) {
      waitCount++
      console.log(`⏳ 等待之前的保存完成... (${waitCount * 100}ms)`)
      await new Promise(resolve => setTimeout(resolve, 100))
    }

    if (isSaving.value) {
      console.warn('⚠️ 等待保存超时，强制保存')
    }

    console.log('💾 开始保存到本地JSON...')
    // 直接调用 saveTokens，不通过 handleSave
    try {
      const jsonString = JSON.stringify(tokens.value, null, 2)
      console.log('📊 保存数据大小:', jsonString.length, '字符')
      await invoke('save_tokens_json', { jsonString })
      console.log('✅ 本地JSON保存完成，tokens数量:', tokens.value.length)
    } catch (saveError) {
      console.error('❌ 保存失败:', saveError)
      window.$notify.error(`保存失败: ${saveError}`)
      throw saveError
    }

    // 如果数据库可用，同步所有数据到数据库
    if (isDatabaseAvailable.value) {
      try {
        console.log('🔄 开始同步所有数据到数据库...')
        console.log(`📊 当前tokens总数: ${tokens.value.length}`)

        // 🔥 关键修改：同步所有tokens到数据库，而不是只同步成功的
        const tokensJson = JSON.stringify(tokens.value)
        await invoke('bidirectional_sync_tokens_with_data', { tokensJson })

        console.log('✅ 数据库同步完成')
      } catch (error) {
        console.error('❌ 同步到数据库失败:', error)
        window.$notify.error(`同步到数据库失败: ${error}`)
      }
    } else {
      console.log('ℹ️ 跳过数据库同步（数据库不可用）')
    }

    // 显示最终汇总
    const failedCount = failedTokens.length

    if (failedCount === 0) {
      // 全部成功
      window.$notify.success(`🎉 已全部完成！成功获取 ${successCount} 个token的额度`)
    } else {
      // 有失败的
      const message = `📊 批量获取完成！\n✅ 成功: ${successCount} 个\n❌ 失败: ${failedCount} 个\n\n失败列表:\n${failedTokens.map(t => `Token ${t.index} (${t.email}): ${t.reason}`).join('\n')}`
      window.$notify.warning(message)
    }
  } catch (error) {
    window.$notify.error(`❌ 批量获取失败: ${error}`)
  } finally {
    isBatchGettingCredits.value = false
  }
}

// 执行去重
const executeDeduplication = async () => {
  if (!duplicateInfo.value.hasDuplicates) {
    window.$notify.warning('没有重复的token')
    return
  }

  isDeduplicating.value = true
  try {
    let deletedCount = 0

    // 遍历所有重复的邮箱
    for (const duplicate of duplicateInfo.value.duplicateEmails) {
      const tokenIds = duplicate.tokenIds
      // 保留第一条，删除其他的
      for (let i = 1; i < tokenIds.length; i++) {
        const tokenId = tokenIds[i]
        // 从内存中删除
        tokens.value = tokens.value.filter(token => token.id !== tokenId)

        // 调用后端删除
        try {
          await invoke('delete_token', { tokenId })
          deletedCount++
        } catch (error) {
          console.error(`删除token ${tokenId} 失败:`, error)
        }
      }
    }

    window.$notify.success(`✅ 去重完成，删除了 ${deletedCount} 条重复token`)
    // 重新检测
    detectDuplicates()
    // 关闭模态框
    showDuplicateModal.value = false
  } catch (error) {
    window.$notify.error(`❌ 去重失败: ${error}`)
  } finally {
    isDeduplicating.value = false
  }
}

// ==================== 多选功能方法 ====================
// 切换多选模式
const toggleSelectionMode = () => {
  selectionMode.value = !selectionMode.value
  if (!selectionMode.value) {
    // 关闭多选模式时清空选中
    selectedTokenIds.value.clear()
  }
}

// 切换单个token的选中状态
const toggleTokenSelection = (tokenId) => {
  if (selectedTokenIds.value.has(tokenId)) {
    selectedTokenIds.value.delete(tokenId)
  } else {
    selectedTokenIds.value.add(tokenId)
  }
}

// 全选/取消全选当前页
const toggleSelectAll = () => {
  if (isCurrentPageAllSelected.value) {
    // 取消全选当前页
    paginatedTokens.value.forEach(token => {
      selectedTokenIds.value.delete(token.id)
    })
  } else {
    // 全选当前页
    paginatedTokens.value.forEach(token => {
      selectedTokenIds.value.add(token.id)
    })
  }
}

// 取消所有选中
const clearSelection = () => {
  selectedTokenIds.value.clear()
}

// 批量复制选中的邮箱
const copySelectedEmails = async () => {
  if (selectedCount.value === 0) {
    window.$notify.warning('请先选择要复制的token')
    return
  }

  const emails = selectedTokens.value
    .filter(token => token.email_note)
    .map(token => token.email_note)

  if (emails.length === 0) {
    window.$notify.warning('选中的token中没有邮箱信息')
    return
  }

  const emailText = emails.join('\n')

  try {
    await navigator.clipboard.writeText(emailText)
    window.$notify.success(`已复制 ${emails.length} 个邮箱`)
  } catch (error) {
    console.error('Failed to copy emails:', error)
    // 降级方案
    const textarea = document.createElement('textarea')
    textarea.value = emailText
    document.body.appendChild(textarea)
    textarea.select()
    document.execCommand('copy')
    document.body.removeChild(textarea)
    window.$notify.success(`已复制 ${emails.length} 个邮箱`)
  }
}

// 批量复制选中的Session
const copySelectedSessions = async () => {
  if (selectedCount.value === 0) {
    window.$notify.warning('请先选择要复制的token')
    return
  }

  const sessions = selectedTokens.value
    .filter(token => token.auth_session)
    .map(token => token.auth_session)

  if (sessions.length === 0) {
    window.$notify.warning('选中的token中没有Session信息')
    return
  }

  const sessionText = sessions.join('\n')

  try {
    await navigator.clipboard.writeText(sessionText)
    window.$notify.success(`已复制 ${sessions.length} 个Session`)
  } catch (error) {
    console.error('Failed to copy sessions:', error)
    // 降级方案
    const textarea = document.createElement('textarea')
    textarea.value = sessionText
    document.body.appendChild(textarea)
    textarea.select()
    document.execCommand('copy')
    document.body.removeChild(textarea)
    window.$notify.success(`已复制 ${sessions.length} 个Session`)
  }
}

// 批量导出选中的tokens
const exportSelectedTokens = async () => {
  if (selectedCount.value === 0) {
    window.$notify.warning('请先选择要导出的token')
    return
  }

  try {
    const tokensToExport = selectedTokens.value

    // 构建导出数据
    const exportData = tokensToExport.map(token => ({
      id: token.id,
      tenant_url: token.tenant_url,
      access_token: token.access_token,
      created_at: token.created_at,
      updated_at: token.updated_at,
      portal_url: token.portal_url,
      email_note: token.email_note,
      tag_name: token.tag_name,
      tag_color: token.tag_color,
      ban_status: token.ban_status,
      portal_info: token.portal_info,
      auth_session: token.auth_session,
      suspensions: token.suspensions,
      balance_color_mode: token.balance_color_mode,
      skip_check: token.skip_check
    }))

    const jsonString = JSON.stringify(exportData, null, 2)

    // 生成文件名
    const now = new Date()
    const dateStr = now.toISOString().split('T')[0]
    const timeStr = now.toTimeString().split(' ')[0].replace(/:/g, '-')
    const fileName = `selected_tokens_${dateStr}_${timeStr}.json`

    // 使用浏览器下载
    const blob = new Blob([jsonString], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = fileName
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)

    window.$notify.success(`已导出 ${exportData.length} 个token`)
  } catch (error) {
    console.error('Export failed:', error)
    window.$notify.error(`导出失败: ${error}`)
  }
}

// 批量删除选中的tokens
const deleteSelectedTokens = async () => {
  if (selectedCount.value === 0) {
    window.$notify.warning('请先选择要删除的token')
    return
  }

  if (!confirm(`确定要删除选中的 ${selectedCount.value} 个token吗？此操作不可撤销！`)) {
    return
  }

  try {
    const tokenIdsToDelete = Array.from(selectedTokenIds.value)

    for (const tokenId of tokenIdsToDelete) {
      // 从内存中删除
      tokens.value = tokens.value.filter(token => token.id !== tokenId)

      // 异步删除后端数据
      try {
        await invoke('delete_token', { tokenId })
      } catch (error) {
        console.error(`删除token ${tokenId} 失败:`, error)
      }
    }

    window.$notify.success(`已删除 ${tokenIdsToDelete.length} 个token`)

    // 清空选中
    clearSelection()

    // 重新检测重复
    nextTick(() => {
      detectDuplicates()
    })
  } catch (error) {
    console.error('Delete failed:', error)
    window.$notify.error(`删除失败: ${error}`)
  }
}

// 删除token
const deleteToken = (tokenId) => {
  const tokenIndex = tokens.value.findIndex(token => token.id === tokenId)
  if (tokenIndex === -1) {
    window.$notify.error(t('messages.tokenNotFound'))
    return
  }

  // 从内存中删除
  tokens.value = tokens.value.filter(token => token.id !== tokenId)
  window.$notify.success(t('messages.tokenDeleted'))

  // 从选中列表中移除
  selectedTokenIds.value.delete(tokenId)

  // 删除后检测重复
  nextTick(() => {
    detectDuplicates()
  })

  // 异步删除后端数据（不阻塞UI）
  invoke('delete_token', { tokenId }).catch(error => {
    console.log('Backend delete failed:', error)
  })
}

// TokenForm event handlers
const handleAddToken = () => {
  editingToken.value = null
  showTokenFormModal.value = true
}

const handleEditToken = (token) => {
  editingToken.value = token
  showTokenFormModal.value = true
}

const closeTokenForm = () => {
  showTokenFormModal.value = false
  editingToken.value = null
}

// 用于标记最后一次添加是否成功
const lastAddTokenSuccess = ref(true)

const handleTokenFormSuccess = () => {
  // 只有在添加成功时才关闭对话框和显示提示
  if (editingToken.value) {
    // 编辑模式总是关闭
    closeTokenForm()
    window.$notify.success(t('messages.tokenUpdated'))
  } else {
    // 添加模式：只有成功时才关闭和提示
    if (lastAddTokenSuccess.value) {
      closeTokenForm()
      window.$notify.success(t('messages.tokenSaved'))
    }
    // 如果失败（重复），不关闭对话框，已经显示了警告并高亮了重复的 token
  }
}

const handleUpdateToken = (updatedTokenData) => {
  const index = tokens.value.findIndex(token => token.id === updatedTokenData.id)
  if (index !== -1) {
    const tagName = updatedTokenData.tagName ? updatedTokenData.tagName.trim() : ''
    const tagColor = updatedTokenData.tagColor || DEFAULT_TAG_COLOR

    // Update the token in the list
    tokens.value[index] = {
      ...tokens.value[index],
      tenant_url: updatedTokenData.tenantUrl,
      access_token: updatedTokenData.accessToken,
      portal_url: updatedTokenData.portalUrl || null,
      email_note: updatedTokenData.emailNote || null,
      tag_name: tagName || null,
      tag_color: tagName ? tagColor : null,
      updated_at: new Date().toISOString()  // 更新 updated_at 时间戳
    }
  }
}

const handleAddTokenFromForm = (tokenData) => {
  const result = addToken(tokenData)
  lastAddTokenSuccess.value = result.success

  // 如果是重复邮箱，高亮并滚动到重复的 token
  if (!result.success && result.duplicateId) {
    highlightAndScrollTo(result.duplicateId)
  }
}

// 处理自动导入完成事件
const handleAutoImportCompleted = () => {
  if (lastAddTokenSuccess.value) {
    // 添加成功,显示成功提示
    window.$notify.success(t('messages.sessionAutoImported'))
  }
  // 无论成功失败都关闭对话框
  closeTokenForm()
}

// 处理手动导入完成事件
const handleManualImportCompleted = () => {
  if (lastAddTokenSuccess.value) {
    // 添加成功,显示成功提示
    window.$notify.success(t('messages.sessionImportSuccess'))
  }
  // 无论成功失败都关闭对话框
  closeTokenForm()
}


// 添加token
const addToken = (tokenData) => {
  // 如果有邮箱，检查是否重复
  if (tokenData.emailNote && tokenData.emailNote.trim()) {
    const emailToCheck = tokenData.emailNote.trim().toLowerCase()
    const duplicate = tokens.value.find(token =>
      token.email_note &&
      token.email_note.trim().toLowerCase() === emailToCheck
    )

    if (duplicate) {
      window.$notify.warning(
        t('messages.duplicateEmailSkipped', { email: tokenData.emailNote })
      )
      return { success: false, duplicateId: duplicate.id }  // 返回重复的 token ID
    }
  }

  const now = new Date().toISOString()
  const tagName = tokenData.tagName ? tokenData.tagName.trim() : ''
  const tagColor = tokenData.tagColor || DEFAULT_TAG_COLOR

  // 构建 portal_info (如果有 creditsBalance 或 expiryDate)
  let portalInfo = null
  if (tokenData.creditsBalance !== undefined && tokenData.creditsBalance !== null) {
    portalInfo = {
      credits_balance: tokenData.creditsBalance,
      expiry_date: tokenData.expiryDate || null
    }
  }

  const newToken = {
    id: crypto.randomUUID(),
    tenant_url: tokenData.tenantUrl,
    access_token: tokenData.accessToken,
    created_at: now,
    updated_at: now,  // 添加 updated_at 字段
    portal_url: tokenData.portalUrl || null,
    ban_status: tokenData.banStatus || null,  // 使用传入的 banStatus，Session 导入时为 'ACTIVE'
    portal_info: portalInfo,  // 使用构建的 portal_info
    email_note: tokenData.emailNote || null,
    tag_name: tagName || null,
    tag_color: tagName ? tagColor : null,
    auth_session: tokenData.authSession || null,  // 添加 auth_session 字段
    suspensions: tokenData.suspensions || null,  // 添加 suspensions 字段
    skip_check: false,  // 默认不跳过检测
    balance_color_mode: null  // 默认为 null，将使用绿色
  }

  tokens.value.push(newToken)

  // 添加后检测重复
  nextTick(() => {
    detectDuplicates()
  })

  return { success: true, token: newToken }
}

// 高亮并滚动到指定 token
const highlightAndScrollTo = (tokenId) => {
  // 清除之前的定时器
  if (highlightTimer) {
    clearTimeout(highlightTimer)
    highlightTimer = null
  }

  // 先清空高亮状态，确保即使是同一个 token 也能重新触发动画
  highlightedTokenId.value = null

  // 查找 token 在 filteredTokens 中的索引
  const tokenIndex = filteredTokens.value.findIndex(token => token.id === tokenId)

  if (tokenIndex === -1) {
    console.warn('Token not found in filtered list:', tokenId)
    return
  }

  // 计算 token 所在的页码
  const targetPage = Math.floor(tokenIndex / pageSize.value) + 1

  // 如果不在当前页,先跳转到目标页
  if (currentPage.value !== targetPage) {
    currentPage.value = targetPage
  }

  // 使用 requestAnimationFrame 确保 DOM 完全更新
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      // 重新设置高亮
      highlightedTokenId.value = tokenId

      nextTick(() => {
        const element = tokenCardRefs.value[tokenId]

        if (element) {
          // 尝试多种方式获取 DOM 元素
          let domElement = null

          // 如果 $el 是文本节点，尝试获取下一个元素节点
          if (element.$el) {
            if (element.$el.nodeType === Node.ELEMENT_NODE) {
              domElement = element.$el
            } else if (element.$el.nextElementSibling) {
              domElement = element.$el.nextElementSibling
            } else if (element.$el.parentElement) {
              // 如果是文本节点，尝试在父元素中查找 .token-card
              domElement = element.$el.parentElement.querySelector('.token-card')
            }
          } else if (element instanceof HTMLElement) {
            domElement = element
          } else if (element.value) {
            domElement = element.value
          }

          if (domElement && typeof domElement.scrollIntoView === 'function') {
            domElement.scrollIntoView({ behavior: 'smooth', block: 'center' })
          }
        }

        // 3秒后取消高亮
        highlightTimer = setTimeout(() => {
          highlightedTokenId.value = null
          highlightTimer = null
        }, 3000)
      })
    })
  })
}

// 打开数据文件夹
const openDataFolder = async () => {
  try {
    await invoke('open_data_folder')
    // 静默执行，不显示状态提示
  } catch (error) {
    window.$notify.error(`${t('bookmarkManager.messages.openFolderFailed')}: ${error}`)
  }
}

// 处理关闭事件
const handleClose = () => {
  // 防抖自动保存会处理保存,直接关闭即可
  emit('close')
}

// 检查当前页账号状态
const checkPageAccountStatus = async () => {
  // 获取当前页需要检测的tokens(过滤掉标记为跳过检测的)
  const tokensToCheck = paginatedTokens.value.filter(token => !token.skip_check)

  if (tokensToCheck.length === 0) {
    return { hasChanges: false }
  }

  try {
    // 准备批量检测的数据
    const tokenInfos = tokensToCheck.map(token => ({
      id: token.id,
      access_token: token.access_token,
      tenant_url: token.tenant_url,
      portal_url: token.portal_url || null,
      auth_session: token.auth_session || null,
      email_note: token.email_note || null
    }))

    // 单次批量API调用检测当前页所有tokens
    const results = await invoke('batch_check_tokens_status', {
      tokens: tokenInfos
    })

    // 批量更新tokens状态
    let hasChanges = false

    results.forEach(result => {
      const token = tokens.value.find(t => t.id === result.token_id)
      if (token) {
        const statusResult = result.status_result
        let tokenHasChanges = false

        // 比对并更新 access_token
        if (token.access_token !== result.access_token) {
          token.access_token = result.access_token
          tokenHasChanges = true
        }

        // 比对并更新 tenant_url
        if (token.tenant_url !== result.tenant_url) {
          token.tenant_url = result.tenant_url
          tokenHasChanges = true
        }

        // 比对并更新 ban_status
        if (token.ban_status !== statusResult.status) {
          token.ban_status = statusResult.status
          tokenHasChanges = true
        }

        // 自动禁用封禁或过期的账号检测
        if ((statusResult.status === 'SUSPENDED' || statusResult.status === 'EXPIRED') && !token.skip_check) {
          token.skip_check = true
          tokenHasChanges = true
          // 显示通知
          const autoDisableMsg = statusResult.status === 'SUSPENDED'
            ? t('messages.autoDisabledBanned')
            : t('messages.autoDisabledExpired')
          window.$notify.info(autoDisableMsg)
        }

        // 比对并更新 suspensions 信息（如果有）
        if (result.suspensions) {
          if (!isEqual(token.suspensions, result.suspensions)) {
            token.suspensions = result.suspensions
            tokenHasChanges = true
            console.log(`Updated suspensions for token ${token.id}:`, result.suspensions)
          }
        }

        // 比对并更新 Portal 信息（如果有）
        if (result.portal_info) {
          const newPortalInfo = {
            credits_balance: result.portal_info.credits_balance,
            expiry_date: result.portal_info.expiry_date
          }

          if (!isEqual(token.portal_info, newPortalInfo)) {
            token.portal_info = newPortalInfo
            tokenHasChanges = true
            console.log(`Updated token ${token.id} portal info:`, token.portal_info)
          }
        } else if (result.portal_error) {
          // 如果获取Portal信息失败，记录错误但不影响状态更新
          console.warn(`Failed to fetch portal info for token ${token.id}:`, result.portal_error)
        }

        // 比对并更新绑卡链接（如果有）
        if (result.payment_method_link && token.payment_method_link !== result.payment_method_link) {
          token.payment_method_link = result.payment_method_link
          tokenHasChanges = true
          console.log(`Updated token ${token.id} payment_method_link`)
        }

        // 比对并更新 email_note（如果有）
        if (result.email_note && token.email_note !== result.email_note) {
          token.email_note = result.email_note
          tokenHasChanges = true
        }

        // 只有在有实际变化时才更新时间戳
        if (tokenHasChanges) {
          token.updated_at = new Date().toISOString()
          hasChanges = true
        }
      }
    })

    return { hasChanges }
  } catch (error) {
    console.error('Batch check page error:', error)
    throw error
  }
}

// 处理刷新事件 - 支持当前页或全部
const handleRefresh = async () => {
  showRefreshOptions.value = true
}

// 刷新指定tokens的额度信息
const refreshCreditsForTokens = async (tokensToRefresh) => {
  if (tokensToRefresh.length === 0) {
    return { hasChanges: false }
  }

  try {
    let hasChanges = false

    // 为每个token创建刷新任务
    const refreshTasks = tokensToRefresh.map((token) => async () => {
      try {
        // 调用后端接口获取额度
        const result = await invoke('batch_check_tokens_status', {
          tokens: [{
            id: token.id,
            access_token: token.access_token,
            tenant_url: token.tenant_url,
            portal_url: token.portal_url || null,
            auth_session: token.auth_session || null,
            email_note: token.email_note || null
          }]
        })

        if (result && result.length > 0) {
          const checkResult = result[0]
          const targetToken = tokens.value.find(t => t.id === token.id)

          if (targetToken && checkResult.portal_info) {
            const newPortalInfo = {
              credits_balance: checkResult.portal_info.credits_balance,
              expiry_date: checkResult.portal_info.expiry_date
            }

            if (!isEqual(targetToken.portal_info, newPortalInfo)) {
              targetToken.portal_info = newPortalInfo
              targetToken.updated_at = new Date().toISOString()
              hasChanges = true
              console.log(`Updated token ${token.id} credits:`, newPortalInfo)
            }
          }
        }
      } catch (error) {
        console.warn(`Failed to refresh credits for token ${token.id}:`, error)
      }
    })

    // 并发执行（最多5个同时进行）
    await executeWithConcurrency(refreshTasks, 5)

    return { hasChanges }
  } catch (error) {
    console.error('Refresh credits error:', error)
    return { hasChanges: false }
  }
}

// 执行刷新操作
const executeRefresh = async (mode = 'current') => {
  if (isRefreshing.value) return
  isRefreshing.value = true
  showRefreshOptions.value = false

  try {
    window.$notify.info(t('messages.refreshingTokenStatus'))

    // 加载最新的 tokens
    await loadTokens(false)
    await nextTick()

    let result

    if (mode === 'current') {
      // 只检查当前页的账号状态
      if (paginatedTokens.value.length > 0) {
        result = await checkPageAccountStatus()
      } else {
        window.$notify.warning(t('messages.noTokensToCheck'))
        return
      }
    } else {
      // 检查全部账号状态
      if (tokens.value.length > 0) {
        result = await checkAllAccountStatus()
      } else {
        window.$notify.warning(t('messages.noTokensToCheck'))
        return
      }
    }

    // 刷新完账号状态后，再刷新额度信息
    let creditsRefreshResult = { hasChanges: false }
    if (mode === 'current') {
      // 刷新当前页的额度
      const tokensToRefreshCredits = paginatedTokens.value.filter(token => !token.skip_check)
      creditsRefreshResult = await refreshCreditsForTokens(tokensToRefreshCredits)
    } else {
      // 刷新全部的额度
      const tokensToRefreshCredits = tokens.value.filter(token => !token.skip_check)
      creditsRefreshResult = await refreshCreditsForTokens(tokensToRefreshCredits)
    }

    // 只有在有实际变化时才保存和标记未同步
    if (result.hasChanges || creditsRefreshResult.hasChanges) {
      // 刷新完成后手动保存更新的状态
      await handleSave()

      // 如果是双向存储模式，标记需要同步
      if (isDatabaseAvailable.value) {
        isSyncNeeded.value = true
      }
    }

    window.$notify.success(t('messages.refreshComplete'))
  } catch (error) {
    window.$notify.error(`${t('messages.refreshFailed')}: ${error.message || error}`)
  } finally {
    // 延迟重置 isRefreshing，确保 watchDebounced 的 debounce timer 已经被清除
    // watchDebounced 的 debounce 时间是 2000ms，这里等待 2100ms 确保安全
    await new Promise(resolve => setTimeout(resolve, 2100))
    isRefreshing.value = false
  }
}



const handleDatabaseConfigSaved = async () => {
  window.$notify.success(t('messages.databaseConfigSaved'))
  // 重新获取存储状态
  await getStorageStatus()
  // 自动执行刷新操作
  await handleRefresh()
}

const handleDatabaseConfigDeleted = async () => {
  window.$notify.info(t('messages.databaseConfigDeleted'))
  // 重新获取存储状态
  await getStorageStatus()
}



// 手动保存方法（显示提示）
const handleManualSave = async () => {
  if (isSaving.value) {
    window.$notify.warning('正在保存中，请稍候...')
    return
  }

  isSaving.value = true
  try {
    console.log('💾 手动保存：开始保存到本地JSON，tokens数量:', tokens.value.length)
    await saveTokens(false)
    console.log('✅ 手动保存：保存成功')
    window.$notify.success(`保存成功！共 ${tokens.value.length} 个Token`)
  } catch (error) {
    console.error('❌ 手动保存失败:', error)
    window.$notify.error(`保存失败: ${error}`)
  } finally {
    isSaving.value = false
  }
}

// 自动保存方法（静默保存，不显示提示）
// 只做本地保存，不触发同步
const handleSave = async () => {
  if (isSaving.value) return

  isSaving.value = true
  try {
    await saveTokens(false)
  } catch (error) {
    // 保存失败时抛出错误，由调用方处理
    throw error
  } finally {
    isSaving.value = false
  }
}

// 强制推送到数据库方法（只推送本地数据，不删除数据库已有数据）
const handleForcePushToDatabase = async () => {
  if (isForcePushing.value) return
  if (!isDatabaseAvailable.value) {
    window.$notify.warning(t('messages.databaseNotAvailable'))
    return
  }

  isForcePushing.value = true
  try {
    window.$notify.info('正在强制推送到数据库...')

    console.log('🔥 强制推送：开始推送本地数据到数据库')
    console.log(`📊 当前tokens总数: ${tokens.value.length}`)

    // 🔥 更新所有token的 updated_at 为当前时间
    const now = new Date().toISOString()
    const updatedTokens = tokens.value.map(token => ({
      ...token,
      updated_at: now
    }))

    // 更新内存中的数据
    tokens.value = updatedTokens

    // 保存到本地JSON
    console.log('💾 保存到本地JSON...')
    await saveTokens(false)
    console.log('✅ 本地JSON保存完成')

    // 🔥 强制推送到数据库（根据session匹配，不删除数据库已有数据）
    console.log('🔄 强制推送到数据库...')
    const tokensJson = JSON.stringify(updatedTokens)
    const result = await invoke('force_push_tokens_to_database', { tokensJson })
    console.log('✅ 数据库推送完成:', result)

    window.$notify.success(`强制推送成功！更新 ${result.updated} 个，新增 ${result.inserted} 个Token`)

    if (result.errors && result.errors.length > 0) {
      console.error('❌ 部分推送失败:', result.errors)
      window.$notify.warning(`部分推送失败，请查看控制台`)
    }
  } catch (error) {
    console.error('❌ 强制推送失败:', error)
    window.$notify.error(`强制推送失败: ${error}`)
  } finally {
    isForcePushing.value = false
  }
}

// 双向同步方法（手动触发）
const handleBidirectionalSync = async () => {
  if (isSyncing.value) return
  if (!isDatabaseAvailable.value) {
    window.$notify.warning(t('messages.databaseNotAvailable'))
    return
  }

  isSyncing.value = true
  try {
    window.$notify.info('正在保存本地数据...')

    // 🔥 第一步：先强制保存本地数据到JSON文件
    console.log('🔥 同步前：先保存本地数据到JSON文件')

    // 等待之前的保存完成
    let waitCount = 0
    while (isSaving.value) {
      waitCount++
      console.log(`⏳ 等待之前的保存完成... (${waitCount * 100}ms)`)
      await new Promise(resolve => setTimeout(resolve, 100))
    }

    // 强制保存到本地JSON
    await saveTokens(false)
    console.log('✅ 本地JSON保存完成，tokens数量:', tokens.value.length)

    window.$notify.info(t('messages.syncingData'))

    // 🔥 第二步：执行双向同步（把本地JSON同步到数据库）
    const tokensJson = JSON.stringify(tokens.value)
    console.log('🔄 开始同步到数据库，tokens数量:', tokens.value.length, '数据长度:', tokensJson.length)
    await invoke('bidirectional_sync_tokens_with_data', { tokensJson })
    console.log('✅ 数据库同步完成')

    // 🔥 第三步：不再重新加载，直接使用当前内存中的数据
    // 因为我们已经保存到本地JSON和数据库了，不需要重新加载
    console.log('✅ 同步完成，当前tokens数量:', tokens.value.length)

    // 同步完成，清除同步需求标记
    isSyncNeeded.value = false

    window.$notify.success(t('messages.syncComplete'))
  } catch (error) {
    console.error('❌ 同步失败:', error)
    window.$notify.error(`${t('messages.syncFailed')}: ${error}`)
  } finally {
    isSyncing.value = false
  }
}

// 组件挂载时自动加载tokens和存储状态
onMounted(async () => {
  // 加载分页配置
  loadPageSize()

  // 加载默认输入框数量配置
  defaultInputCount.value = loadDefaultInputCount()

  // 初始化输入框
  initializeSessionInputs(defaultInputCount.value)

  // 首先获取存储状态
  await getStorageStatus()

  // 使用 isLoadingFromSync 标记初始加载，避免触发自动保存
  isLoadingFromSync.value = true
  await loadTokens(false) // 显示成功消息

  // 延迟重置标记，确保 watchDebounced 的 debounce timer 已经被清除
  await new Promise(resolve => setTimeout(resolve, 2100))
  isLoadingFromSync.value = false

  // 初始化时，如果是双向存储模式，默认不标记需要同步
  // 只有在用户修改后才标记
  isSyncNeeded.value = false

  isReady.value = true

  // 监听后端发送的 tokens-updated 事件
  unlistenTokensUpdated = await listen('tokens-updated', async () => {
    console.log('📡 Received tokens-updated event from backend, reloading tokens...')
    await loadTokens(false)
  })
})

// 组件卸载时清理定时器和事件监听器
onUnmounted(() => {
  if (storageCheckTimer) {
    clearInterval(storageCheckTimer)
    storageCheckTimer = null
  }

  // 取消事件监听
  if (unlistenTokensUpdated) {
    unlistenTokensUpdated()
    unlistenTokensUpdated = null
  }
})

// 搜索时重置到第一页
watch(searchQuery, () => {
  currentPage.value = 1
})

// 防抖自动保存 - 监听 tokens 变化
watchDebounced(
  tokens,
  async (newTokens, oldTokens) => {
    // 只有在组件就绪后才自动保存
    if (!isReady.value) return

    // 如果正在保存,跳过
    if (isSaving.value) return

    // 如果正在同步导致的加载,跳过（避免循环触发）
    if (isLoadingFromSync.value) return

    // 如果正在批量刷新或批量获取额度,跳过（完成后会手动保存）
    if (isRefreshing.value || isBatchGettingCredits.value) return

    // 如果tokens为空且之前也为空,跳过
    if (newTokens.length === 0 && (!oldTokens || oldTokens.length === 0)) return

    try {
      await handleSave()
      window.$notify.success(t('messages.autoSaveSuccess'))

      // 如果是双向存储模式，标记需要同步
      if (isDatabaseAvailable.value) {
        isSyncNeeded.value = true
      }
    } catch (error) {
      window.$notify.error(t('messages.autoSaveFailed') + ': ' + error)
    }
  },
  {
    debounce: 2000,  // 2秒防抖
    deep: true       // 深度监听
  }
)

// 暴露方法给父组件
defineExpose({
  addToken,    // 允许App.vue添加token
  deleteToken, // 允许App.vue删除token
  tokens: readonly(tokens), // 只读访问tokens
  saveTokens,   // 允许App.vue保存tokens
  waitUntilReady, // 暴露就绪等待方法
  highlightAndScrollTo // 暴露高亮和滚动方法
})
</script>

<style scoped>
.token-list-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 2000;
}

.modal-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.modal-content {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  width: 95vw;
  /* 使用视口宽度的 95%,自适应屏幕大小 */
  max-width: none;
  /* 移除固定最大宽度限制 */
  height: 90vh;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
}

/* 移除旧的 modal-header 样式，使用新的样式 */

/* 批量操作区域 */
.batch-actions-bar {
  background: linear-gradient(135deg, #f0f4ff 0%, #e8f0fe 100%);
  padding: 12px 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  border-bottom: 1px solid #d1e0ff;
  box-shadow: 0 2px 8px rgba(59, 130, 246, 0.08);
}

.batch-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.selected-count {
  color: #1e40af;
  font-weight: 600;
  font-size: 14px;
  padding: 6px 12px;
  background: #ffffff;
  border-radius: 6px;
  border: 1px solid #bfdbfe;
  box-shadow: 0 1px 3px rgba(59, 130, 246, 0.1);
}

.batch-buttons {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.batch-buttons .btn {
  background: #ffffff;
  color: #374151;
  border: 1px solid #e5e7eb;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}

.batch-buttons .btn:hover {
  background: #f9fafb;
  transform: translateY(-1px);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.12);
  border-color: #d1d5db;
}

.batch-buttons .btn.secondary {
  background: #f3f4f6;
  color: #4b5563;
  border-color: #d1d5db;
}

.batch-buttons .btn.secondary:hover {
  background: #e5e7eb;
  border-color: #9ca3af;
}

.batch-buttons .btn.danger {
  background: #fee2e2;
  color: #dc2626;
  border-color: #fecaca;
}

.batch-buttons .btn.danger:hover {
  background: #fecaca;
  border-color: #fca5a5;
}

.batch-buttons .btn.success {
  background: #d1fae5;
  color: #059669;
  border-color: #a7f3d0;
}

.batch-buttons .btn.success:hover {
  background: #a7f3d0;
  border-color: #6ee7b7;
}

.batch-buttons .btn.info {
  background: #dbeafe;
  color: #2563eb;
  border-color: #bfdbfe;
}

.batch-buttons .btn.info:hover {
  background: #bfdbfe;
  border-color: #93c5fd;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--color-text-muted, #6b7280);
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.close-btn:hover {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-primary, #374151);
}

.modal-body {
  padding: 24px;
  flex: 1;
  overflow-y: auto;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
}

.empty-icon {
  color: var(--color-border-strong, #d1d5db);
  margin-bottom: 16px;
}

.empty-state h3 {
  color: var(--color-text-primary, #374151);
  margin: 0 0 8px 0;
  font-size: 1.25rem;
}

.empty-state p {
  color: var(--color-text-muted, #6b7280);
  margin: 0 0 24px 0;
}

/* 空状态下的批量导入按钮 */
.batch-import-btn-empty {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: var(--color-primary, #2563eb);
  color: #ffffff;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
  font-weight: 500;
  box-shadow: 0 2px 4px rgba(37, 99, 235, 0.2);
}

.batch-import-btn-empty:hover {
  background: var(--color-primary-hover, #1d4ed8);
  box-shadow: 0 4px 8px rgba(37, 99, 235, 0.3);
  transform: translateY(-1px);
}

.batch-import-btn-empty:active {
  transform: translateY(0);
  box-shadow: 0 2px 4px rgba(37, 99, 235, 0.2);
}

.batch-import-btn-empty svg {
  flex-shrink: 0;
}

.loading-state {
  text-align: center;
  padding: 40px 20px;
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--color-border, #e5e7eb);
  border-top: 3px solid var(--color-accent, #3b82f6);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 16px;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}

.loading-state p {
  color: var(--color-text-muted, #6b7280);
  margin: 0;
}

.token-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(330px, 1fr));
  gap: 16px;
  padding: 4px;
}

/* 滚动按钮 */
.scroll-buttons {
  position: fixed;
  right: 24px;
  bottom: 24px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  z-index: 100;
}

.scroll-btn {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  border: 1px solid var(--color-border, #e5e7eb);
  background: var(--color-background, #ffffff);
  color: var(--color-text, #374151);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.scroll-btn:hover {
  background: var(--color-primary, #2563eb);
  border-color: var(--color-primary, #2563eb);
  color: #ffffff;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(37, 99, 235, 0.3);
}

.scroll-btn:active {
  transform: translateY(0);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

/* 响应式处理 */

/* 超大屏幕优化 */
@media (min-width: 1920px) {
  .token-grid {
    /* 超大屏幕: 每列最小 320px,允许更多列 */
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 20px;
  }
}

/* 中等屏幕 */
@media (max-width: 768px) {
  .modal-content {
    margin: 10px;
    width: calc(100vw - 20px);
  }

  .modal-header {
    padding: 16px;
  }

  .modal-body {
    padding: 16px;
  }

  .header-actions {
    gap: 6px;
  }

  .token-grid {
    grid-template-columns: 1fr;
    gap: 12px;
  }

  .list-toolbar {
    flex-wrap: wrap;
    gap: 8px;
  }

  .search-box {
    min-width: 150px;
  }
}

/* 工具栏按钮统一样式 */
.open-folder-btn,
.batch-delete-btn,
.batch-import-btn,
.sort-dropdown .sort-btn {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  height: 36px;
  padding: 0 12px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
  background: var(--color-background, #ffffff);
  color: var(--color-text, #374151);
  cursor: pointer;
  transition: all 0.2s;
}

.open-folder-btn:hover {
  background: var(--color-background-soft, #f9fafb);
  border-color: var(--color-primary, #2563eb);
}

.batch-delete-btn:hover:not(:disabled) {
  background: var(--color-background-soft, #f9fafb);
  border-color: var(--color-danger, #dc2626);
  color: var(--color-danger, #dc2626);
}

.batch-delete-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.export-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 12px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
  background: var(--color-background, #ffffff);
  color: var(--color-text-secondary, #6b7280);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.export-btn:hover:not(:disabled) {
  background: var(--color-background-soft, #f9fafb);
  border-color: var(--color-success, #10b981);
  color: var(--color-success, #10b981);
}

.export-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.copy-emails-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 12px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
  background: var(--color-background, #ffffff);
  color: var(--color-text-secondary, #6b7280);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.copy-emails-btn:hover:not(:disabled) {
  background: var(--color-background-soft, #f9fafb);
  border-color: var(--color-info, #0ea5e9);
  color: var(--color-info, #0ea5e9);
}

.copy-emails-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.batch-import-btn:hover {
  background: var(--color-background-soft, #f9fafb);
  border-color: var(--color-primary, #2563eb);
}

/* 批量导入对话框 */
.batch-import-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  padding: 20px;
}

.batch-import-dialog {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
  max-width: 896px;
  /* 2xl: 56rem = 896px */
  width: 100%;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.batch-import-dialog .dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
}

.batch-import-dialog .dialog-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary, #374151);
}

.batch-import-dialog .dialog-close {
  background: none;
  border: none;
  padding: 4px;
  cursor: pointer;
  color: var(--color-text-muted, #6b7280);
  border-radius: 4px;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.batch-import-dialog .dialog-close:hover {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-primary, #374151);
}

/* Tab Navigation */
.batch-import-tabs {
  display: flex;
  gap: 0;
  padding: 0 24px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
  background: var(--color-surface-alt, #f9fafb);
}

.batch-import-tab {
  padding: 12px 20px;
  border: none;
  background: transparent;
  color: var(--color-text-secondary, #6b7280);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  border-bottom: 2px solid transparent;
  position: relative;
}

.batch-import-tab:hover {
  color: var(--color-text-primary, #374151);
  background: var(--color-surface-hover, #f3f4f6);
}

.batch-import-tab.active {
  color: var(--color-primary, #2563eb);
  border-bottom-color: var(--color-primary, #2563eb);
}

.batch-import-tab svg {
  flex-shrink: 0;
}

.batch-import-dialog .dialog-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.tab-content {
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }

  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.batch-import-dialog .dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid var(--color-divider, #e1e5e9);
  background: var(--color-surface, #ffffff);
}

.batch-import-dialog .btn-cancel {
  padding: 8px 16px;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 6px;
  background: var(--color-surface, #ffffff);
  color: var(--color-text-primary, #374151);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.batch-import-dialog .btn-cancel:hover {
  background: var(--color-surface-hover, #f3f4f6);
  border-color: var(--color-border-hover, #9ca3af);
}

.batch-import-dialog .btn-confirm {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  background: var(--color-primary, #2563eb);
  color: #ffffff;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.batch-import-dialog .btn-confirm:hover:not(:disabled) {
  background: var(--color-primary-hover, #1d4ed8);
}

.batch-import-dialog .btn-confirm:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.import-textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 8px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 13px;
  line-height: 1.5;
  resize: vertical;
  background: var(--color-surface, #ffffff);
  color: var(--color-text-primary, #374151);
  transition: border-color 0.2s ease;
}

.import-textarea:focus {
  outline: none;
  border-color: var(--color-primary, #2563eb);
}

.import-textarea::placeholder {
  color: var(--color-text-muted, #9ca3af);
}

.format-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
  margin-bottom: 20px;
}

.format-option-single {
  padding: 16px;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 8px;
  background: var(--color-surface-secondary, #f9fafb);
  margin-bottom: 16px;
}

.format-option {
  padding: 16px;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 8px;
  background: var(--color-surface-alt, #f9fafb);
}

.format-header {
  margin-bottom: 8px;
}

.format-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary, #374151);
}

.format-desc {
  font-size: 13px;
  color: var(--color-text-secondary, #6b7280);
  margin: 0 0 12px 0;
  line-height: 1.5;
}

.btn-fill-template {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--color-primary, #2563eb);
  border-radius: 6px;
  background: var(--color-surface, #ffffff);
  color: var(--color-primary, #2563eb);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-fill-template:hover {
  background: var(--color-primary, #2563eb);
  color: #ffffff;
}

.import-input-section {
  margin: 16px 0;
}

.import-errors {
  margin-top: 16px;
  padding: 12px;
  background: var(--color-danger-light, #fee2e2);
  border: 1px solid var(--color-danger, #dc2626);
  border-radius: 8px;
}

.import-errors .error-header {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--color-danger, #dc2626);
  font-weight: 600;
  margin-bottom: 8px;
}

.import-errors .error-list {
  margin: 0;
  padding-left: 24px;
  color: var(--color-danger, #dc2626);
  font-size: 13px;
}

.import-errors .error-list li {
  margin: 4px 0;
}

.import-preview {
  margin-top: 16px;
  padding: 12px;
  background: var(--color-success-light, #d1fae5);
  border: 1px solid var(--color-success, #10b981);
  border-radius: 8px;
}

.import-preview .preview-header {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--color-success, #10b981);
  font-weight: 600;
}

/* Session 动态输入框样式 */
.session-inputs-container {
  margin-bottom: 16px;
}

.session-input-item {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.session-input-number {
  flex-shrink: 0;
  width: 24px;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-secondary, #6b7280);
  text-align: right;
}

.session-input-field {
  flex: 1;
  height: 40px;
  padding: 0 12px;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 6px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 13px;
  background: var(--color-surface, #ffffff);
  color: var(--color-text-primary, #374151);
  transition: all 0.2s;
}

.session-input-field:focus {
  outline: none;
  border-color: var(--color-primary, #2563eb);
  box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
}

.session-input-field::placeholder {
  color: var(--color-text-muted, #9ca3af);
}

.session-input-delete {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  padding: 0;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 6px;
  background: var(--color-surface, #ffffff);
  color: var(--color-text-muted, #6b7280);
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.session-input-delete:hover:not(:disabled) {
  background: var(--color-danger-light, #fee2e2);
  border-color: var(--color-danger, #dc2626);
  color: var(--color-danger, #dc2626);
}

.session-input-delete:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.add-more-btn {
  width: 100%;
  padding: 10px 16px;
  border: 2px dashed var(--color-divider, #e1e5e9);
  border-radius: 6px;
  background: transparent;
  color: var(--color-text-secondary, #6b7280);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.add-more-btn:hover {
  border-color: var(--color-primary, #2563eb);
  color: var(--color-primary, #2563eb);
  background: rgba(37, 99, 235, 0.05);
}

.add-more-btn svg {
  flex-shrink: 0;
}

/* Session 导入模式切换 */
.session-mode-toggle {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
  padding-bottom: 12px;
}

.mode-btn {
  flex: 1;
  padding: 8px 12px;
  border: none;
  background: transparent;
  color: var(--color-text-secondary, #6b7280);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  border-bottom: 2px solid transparent;
  margin-bottom: -13px;
}

.mode-btn:hover {
  color: var(--color-text-primary, #374151);
}

.mode-btn.active {
  color: var(--color-primary, #2563eb);
  border-bottom-color: var(--color-primary, #2563eb);
}

.mode-btn svg {
  flex-shrink: 0;
}

/* 多行粘贴模式样式 */
.session-multi-mode {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.session-batch-textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 6px;
  background: var(--color-surface, #ffffff);
  color: var(--color-text-primary, #374151);
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 13px;
  line-height: 1.5;
  resize: vertical;
  transition: all 0.2s;
}

.session-batch-textarea:focus {
  outline: none;
  border-color: var(--color-primary, #2563eb);
  box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
}

.session-batch-textarea::placeholder {
  color: var(--color-text-muted, #9ca3af);
}

/* Session 预览 - 简化版 */
.session-preview-simple {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  border: 1px solid var(--color-success-border, #d1fae5);
  border-radius: 6px;
  background: var(--color-success-light, #f0fdf4);
  margin-top: 8px;
}

.preview-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--color-success-text, #065f46);
}

.preview-info svg {
  flex-shrink: 0;
  color: var(--color-success, #10b981);
}

.preview-info strong {
  font-weight: 600;
  color: var(--color-success, #10b981);
}

/* 黑暗模式 */
[data-theme='dark'] .session-batch-textarea {
  background: var(--color-surface, #1f2937);
  color: var(--color-text-primary, #f3f4f6);
  border-color: rgba(75, 85, 99, 0.6);
}

[data-theme='dark'] .session-batch-textarea:focus {
  border-color: var(--color-primary, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

[data-theme='dark'] .session-preview-simple {
  background: rgba(5, 150, 105, 0.1);
  border-color: rgba(5, 150, 105, 0.3);
}

[data-theme='dark'] .preview-info {
  color: #a7f3d0;
}

[data-theme='dark'] .preview-info svg {
  color: #10b981;
}

[data-theme='dark'] .preview-info strong {
  color: #10b981;
}

/* 右键菜单 */
.context-menu-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 20000;
}

.context-menu {
  position: fixed;
  background: var(--color-surface, #ffffff);
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 20001;
  min-width: 180px;
  overflow: hidden;
}

.context-menu-header {
  padding: 8px 12px;
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-muted, #6b7280);
  background: var(--color-surface-hover, #f3f4f6);
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
}

.context-menu-item {
  padding: 8px 16px;
  cursor: pointer;
  color: var(--color-text-primary, #374151);
  transition: background 0.2s ease;
  font-size: 14px;
}

.context-menu-item:hover {
  background: var(--color-primary-light, #e0f2fe);
  color: var(--color-primary, #0ea5e9);
}

/* 右键菜单操作项样式 */
.context-menu-action {
  display: flex;
  align-items: center;
  font-weight: 500;
  color: var(--color-primary, #0ea5e9);
}

.context-menu-action:hover {
  background: var(--color-primary-light, #e0f2fe);
  color: var(--color-primary-dark, #0284c7);
}

.context-menu-action svg {
  flex-shrink: 0;
}

.context-menu-divider {
  height: 1px;
  background: var(--color-divider, #e1e5e9);
  margin: 4px 0;
}

.context-menu-custom {
  padding: 8px 12px;
  display: flex;
  gap: 8px;
  align-items: center;
}

.custom-count-input {
  flex: 1;
  padding: 6px 8px;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 4px;
  font-size: 14px;
  color: var(--color-text-primary, #374151);
  background: var(--color-surface, #ffffff);
  outline: none;
  transition: border-color 0.2s ease;
}

.custom-count-input:focus {
  border-color: var(--color-primary, #0ea5e9);
}

.custom-count-input::placeholder {
  color: var(--color-text-muted, #9ca3af);
}

.btn-custom-fill {
  padding: 6px 12px;
  background: var(--color-primary, #0ea5e9);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s ease;
  white-space: nowrap;
}

.btn-custom-fill:hover {
  background: var(--color-primary-dark, #0284c7);
}

/* 批量删除对话框 */
.batch-delete-overlay {
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

.batch-delete-dialog {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  max-width: 500px;
  width: 100%;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
}

.batch-delete-dialog .dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
}

.batch-delete-dialog .dialog-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary, #374151);
}

.batch-delete-dialog .dialog-close {
  background: none;
  border: none;
  padding: 4px;
  cursor: pointer;
  color: var(--color-text-muted, #6b7280);
  border-radius: 4px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.batch-delete-dialog .dialog-close:hover {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-primary, #374151);
}

.batch-delete-dialog .dialog-body {
  padding: 24px;
}

.dialog-message {
  margin: 0 0 16px 0;
  color: var(--color-text-secondary, #6b7280);
  font-size: 14px;
  white-space: pre-line;
  line-height: 1.6;
}

.delete-stats {
  background: var(--color-surface-secondary, #f9fafb);
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 16px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
}

.stat-item:not(:last-child) {
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
}

.stat-item.total {
  font-weight: 600;
  color: var(--color-text-primary, #374151);
}

.stat-label {
  color: var(--color-text-secondary, #6b7280);
  font-size: 14px;
}

.stat-value {
  color: var(--color-text-primary, #374151);
  font-size: 14px;
  font-weight: 500;
}

.dialog-warning {
  margin: 0;
  color: var(--color-warning-text, #92400e);
  background: var(--color-warning-surface, #fef3c7);
  border: 1px solid var(--color-warning-border, #fde68a);
  border-radius: 6px;
  padding: 12px;
  font-size: 13px;
}

.batch-delete-dialog .dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid var(--color-divider, #e1e5e9);
  background: var(--color-surface, #ffffff);
}

.btn-danger {
  background: var(--color-danger, #dc2626);
  color: white;
  border: 1px solid var(--color-danger, #dc2626);
}

.btn-danger:hover:not(:disabled) {
  background: var(--color-danger-hover, #b91c1c);
  border-color: var(--color-danger-hover, #b91c1c);
}

.btn-danger:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* 黑暗模式 */
[data-theme='dark'] .batch-import-dialog {
  background: var(--color-surface, #1f2937);
}

[data-theme='dark'] .batch-import-dialog .dialog-footer {
  background: var(--color-surface, #1f2937);
}

/* 余额筛选 */
.balance-filter-dropdown {
  position: relative;
  display: inline-block;
}

.balance-filter-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--color-surface-hover, #f3f4f6);
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  color: var(--color-text-primary, #374151);
  transition: all 0.2s ease;
}

.balance-filter-btn:hover {
  background: var(--color-surface-active, #e5e7eb);
  border-color: var(--color-primary, #3b82f6);
}

.balance-filter-btn.active {
  background: var(--color-primary-light, #dbeafe);
  border-color: var(--color-primary, #3b82f6);
  color: var(--color-primary, #3b82f6);
}

.filter-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  background: var(--color-success, #10b981);
  color: white;
  border-radius: 50%;
  font-size: 12px;
  font-weight: bold;
}

.balance-filter-panel {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 8px;
  background: var(--color-surface, #ffffff);
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  min-width: 280px;
}

.filter-panel-header {
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
}

.filter-panel-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary, #374151);
}

.filter-panel-body {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.filter-input-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.filter-input-group label {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text-primary, #374151);
}

.filter-input {
  padding: 8px 12px;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 6px;
  font-size: 14px;
  color: var(--color-text-primary, #374151);
  background: var(--color-surface, #ffffff);
  transition: all 0.2s ease;
}

.filter-input:focus {
  outline: none;
  border-color: var(--color-primary, #3b82f6);
  box-shadow: 0 0 0 3px var(--color-primary-light, #dbeafe);
}

.filter-input::placeholder {
  color: var(--color-text-muted, #9ca3af);
}

.filter-panel-footer {
  padding: 12px 16px;
  border-top: 1px solid var(--color-divider, #e1e5e9);
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

.btn.small {
  padding: 6px 12px;
  font-size: 13px;
}

/* 黑暗模式 - 余额筛选 */
[data-theme='dark'] .balance-filter-panel {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-divider, #374151);
}

[data-theme='dark'] .filter-input {
  background: var(--color-surface-hover, #111827);
  border-color: var(--color-divider, #374151);
  color: var(--color-text-primary, #f3f4f6);
}

[data-theme='dark'] .filter-input:focus {
  border-color: var(--color-primary, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
}

/* 导出对话框 */
.export-overlay {
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

.export-dialog {
  background: var(--color-surface, #ffffff);
  border-radius: 12px;
  max-width: 500px;
  width: 100%;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.2);
}

.export-dialog .dialog-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-divider, #e1e5e9);
}

.export-dialog .dialog-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary, #374151);
}

.export-dialog .dialog-close {
  background: none;
  border: none;
  padding: 4px;
  cursor: pointer;
  color: var(--color-text-muted, #6b7280);
  border-radius: 4px;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.export-dialog .dialog-close:hover {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-primary, #374151);
}

.export-dialog .dialog-body {
  padding: 24px;
  max-height: 60vh;
  overflow-y: auto;
}

.export-options {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.option-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.option-label {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary, #374151);
  display: flex;
  align-items: center;
  gap: 8px;
}

.option-label input[type='checkbox'] {
  cursor: pointer;
  width: 16px;
  height: 16px;
}

.export-count-options {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.count-btn {
  padding: 8px 12px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
  background: var(--color-background, #ffffff);
  color: var(--color-text, #374151);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.count-btn:hover:not(:disabled) {
  border-color: var(--color-primary, #2563eb);
  background: var(--color-background-soft, #f9fafb);
}

.count-btn.active {
  background: var(--color-primary, #2563eb);
  color: #ffffff;
  border-color: var(--color-primary, #2563eb);
}

.count-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.option-warning {
  margin: 0;
  padding: 8px 12px;
  background: var(--color-warning-light, #fef3c7);
  border: 1px solid var(--color-warning, #f59e0b);
  border-radius: 6px;
  font-size: 13px;
  color: var(--color-warning, #f59e0b);
  line-height: 1.5;
}

.export-stats {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px;
  background: var(--color-background-soft, #f9fafb);
  border-radius: 8px;
}

.export-stats .stat-item {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.export-stats .stat-item.warning {
  color: var(--color-warning, #f59e0b);
  font-weight: 600;
}

.export-dialog .dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid var(--color-divider, #e1e5e9);
}

.btn {
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn.primary {
  background: var(--color-primary, #2563eb);
  color: #ffffff;
}

.btn.primary:hover:not(:disabled) {
  background: var(--color-primary-dark, #1d4ed8);
}

.btn.secondary {
  background: var(--color-background-soft, #f9fafb);
  color: var(--color-text, #374151);
  border: 1px solid var(--color-border, #e5e7eb);
}

.btn.secondary:hover:not(:disabled) {
  background: var(--color-background, #ffffff);
  border-color: var(--color-primary, #2563eb);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

[data-theme='dark'] .batch-delete-dialog {
  background: var(--color-surface, #1f2937);
}

[data-theme='dark'] .export-dialog {
  background: var(--color-surface, #1f2937);
}

[data-theme='dark'] .delete-stats {
  background: rgba(55, 65, 81, 0.5);
  border-color: rgba(75, 85, 99, 0.6);
}

[data-theme='dark'] .dialog-warning {
  background: rgba(245, 158, 11, 0.2);
  border-color: rgba(245, 158, 11, 0.4);
  color: #fbbf24;
}

[data-theme='dark'] .format-option,
[data-theme='dark'] .format-option-single {
  background: rgba(55, 65, 81, 0.3);
  border-color: rgba(75, 85, 99, 0.6);
}

[data-theme='dark'] .btn-fill-template {
  background: rgba(37, 99, 235, 0.1);
  border-color: var(--color-primary, #3b82f6);
  color: var(--color-primary, #3b82f6);
}

[data-theme='dark'] .btn-fill-template:hover {
  background: var(--color-primary, #3b82f6);
  color: #ffffff;
}

@media (max-width: 480px) {
  .modal-overlay {
    padding: 10px;
  }

  .modal-content {
    max-height: 95vh;
  }

  .modal-header h2 {
    font-size: 1.25rem;
  }

  .empty-state {
    padding: 20px 10px;
  }

  .empty-state h3 {
    font-size: 1.125rem;
  }

  .btn.small {
    padding: 4px 8px;
    font-size: 11px;
  }

  .sync-actions {
    flex-direction: column;
  }

  .btn.sync-btn {
    min-width: auto;
  }
}

.list-header {
  margin-bottom: 16px;
}

.list-toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--color-background-soft, #f9fafb);
  border-radius: 8px;
}

/* 搜索框样式 */
.search-box {
  position: relative;
  display: flex;
  align-items: center;
  flex: 1;
  max-width: 400px;
  min-width: 200px;
}

.search-icon {
  position: absolute;
  left: 10px;
  color: var(--color-text-secondary, #6b7280);
  pointer-events: none;
}

.search-input {
  width: 300px;
  height: 36px;
  padding: 0 32px;
  border: 1px solid var(--color-divider, #e1e5e9);
  border-radius: 6px;
  font-size: 13px;
  color: var(--color-text-primary, #374151);
  background: var(--color-surface, #ffffff);
  transition: all 0.2s ease;
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.search-input::placeholder {
  color: var(--color-text-secondary, #9ca3af);
}

.clear-search-btn {
  position: absolute;
  right: 6px;
  padding: 4px;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--color-text-secondary, #6b7280);
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.clear-search-btn:hover {
  background: var(--color-hover, #f3f4f6);
  color: var(--color-text-primary, #374151);
}

/* 无搜索结果样式 */
.no-search-results {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  color: var(--color-text-secondary, #6b7280);
}

.no-search-results p {
  margin-top: 16px;
  font-size: 14px;
}

/* 排序下拉菜单 */
.sort-dropdown {
  position: relative;
}

.sort-dropdown .sort-btn:hover {
  background: var(--color-background-soft, #f9fafb);
  border-color: var(--color-primary, #2563eb);
}

/* 下拉菜单容器 */
.sort-menu {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  min-width: 200px;
  background: var(--color-background, #ffffff);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  overflow: hidden;
}

/* 下拉菜单选项 */
.sort-option {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 10px 12px;
  border: none;
  background: transparent;
  color: var(--color-text, #374151);
  cursor: pointer;
  transition: background 0.2s;
  text-align: left;
}

.sort-option:hover {
  background: var(--color-background-soft, #f9fafb);
}

.sort-option.active {
  background: var(--color-primary-soft, rgba(37, 99, 235, 0.1));
  color: var(--color-primary, #2563eb);
}

.sort-option span {
  flex: 1;
  font-size: 14px;
}

.sort-option .arrow-down,
.sort-option .arrow-up {
  opacity: 0.5;
}

.sort-option .check-icon {
  color: var(--color-primary, #2563eb);
}

/* 分隔线 */
.sort-divider {
  height: 1px;
  background: var(--color-border, #e5e7eb);
  margin: 4px 0;
}

/* 下拉菜单动画 */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.2s ease;
}

.dropdown-enter-from {
  opacity: 0;
  transform: translateY(-8px);
}

.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}

/* 分页控制 */
.pagination-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--color-background-soft, #f9fafb);
  border-radius: 8px;
  margin-bottom: 16px;
}

.pagination-info {
  font-size: 14px;
  color: var(--color-text, #374151);
}

.pagination-size {
  display: flex;
  align-items: center;
  gap: 8px;
}

.pagination-size label {
  font-size: 14px;
  color: var(--color-text, #374151);
}

.pagination-size select {
  padding: 4px 8px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 4px;
  background: var(--color-background, #ffffff);
  color: var(--color-text, #374151);
  cursor: pointer;
}

/* 分页导航 */
.pagination-nav {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 16px;
  padding: 16px;
  margin-top: 16px;
}

.pagination-btn {
  padding: 8px 16px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 4px;
  background: var(--color-background, #ffffff);
  color: var(--color-text, #374151);
  cursor: pointer;
  transition: all 0.2s;
}

.pagination-btn:hover:not(:disabled) {
  background: var(--color-background-soft, #f9fafb);
  border-color: var(--color-primary, #2563eb);
}

.pagination-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.pagination-pages {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--color-text, #374151);
}

.page-number {
  min-width: 32px;
  height: 32px;
  padding: 0 8px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 4px;
  background: var(--color-background, #ffffff);
  color: var(--color-text, #374151);
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
}

.page-number:hover {
  background: var(--color-background-soft, #f9fafb);
  border-color: var(--color-primary, #2563eb);
  color: var(--color-primary, #2563eb);
}

.page-number.active {
  background: var(--color-primary, #2563eb);
  border-color: var(--color-primary, #2563eb);
  color: #ffffff;
  font-weight: 600;
}

.page-ellipsis {
  padding: 0 4px;
  color: var(--color-text-secondary, #6b7280);
}

.page-jump {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--color-text, #374151);
}

.page-jump-input {
  width: 60px;
  height: 32px;
  padding: 0 8px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 4px;
  background: var(--color-background, #ffffff);
  color: var(--color-text, #374151);
  text-align: center;
  font-size: 14px;
}

.page-jump-input:focus {
  outline: none;
  border-color: var(--color-primary, #2563eb);
}

.page-current {
  font-weight: 600;
  color: var(--color-primary, #2563eb);
}

/* 工具栏内联分页信息和选择器 */
.pagination-combined {
  display: flex;
  align-items: center;
  gap: 12px;
  height: 36px;
  padding: 0 12px;
  background: var(--color-background, #ffffff);
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 6px;
}

.pagination-info-text {
  font-size: 13px;
  color: var(--color-text-secondary, #6b7280);
  white-space: nowrap;
}

.pagination-size-select {
  height: 28px;
  padding: 0 8px;
  border: 1px solid var(--color-border, #e5e7eb);
  border-radius: 4px;
  background: var(--color-background, #ffffff);
  color: var(--color-text, #374151);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.pagination-size-select:hover {
  border-color: var(--color-primary, #2563eb);
}

.pagination-size-select:focus {
  outline: none;
  border-color: var(--color-primary, #2563eb);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.btn {
  padding: 8px 16px;
  border: 1px solid transparent;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  height: 36px;
  box-sizing: border-box;
}

.btn.secondary {
  background: var(--color-surface-hover, #f3f4f6);
  color: var(--color-text-primary, #374151);
  border: 1px solid var(--color-border-strong, #d1d5db);
}

.btn.secondary:hover {
  background: var(--color-border, #e5e7eb);
  border-color: var(--color-border-hover, #9ca3af);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(55, 65, 81, 0.2);
}

.btn.success {
  background: var(--color-success-surface, #d1fae5);
  color: var(--color-success-text, #065f46);
  border: 1px solid var(--color-success-border, #a7f3d0);
}

.btn.success:hover:not(:disabled) {
  background: var(--color-success-surface, #a7f3d0);
  border-color: var(--color-success-border, #6ee7b7);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(6, 95, 70, 0.3);
}

.btn.success:disabled {
  background: var(--color-border-strong, #d1d5db);
  color: var(--color-text-soft, #9ca3af);
  border-color: var(--color-border-strong, #d1d5db);
  cursor: not-allowed;
}

.btn.warning {
  background: #fef3c7;
  color: #92400e;
  border: 1px solid #fbbf24;
}

.btn.warning:hover:not(:disabled) {
  background: #fde68a;
  border-color: #f59e0b;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(251, 191, 36, 0.3);
}

.btn.warning:disabled {
  background: var(--color-border-strong, #d1d5db);
  color: var(--color-text-soft, #9ca3af);
  border-color: var(--color-border-strong, #d1d5db);
  cursor: not-allowed;
}

.btn.info {
  background: var(--color-info-surface, #dbeafe);
  color: var(--color-info-text, #1e40af);
  border: 1px solid var(--color-info-border, #93c5fd);
}

.btn.info:hover:not(:disabled) {
  background: var(--color-info-surface, #bfdbfe);
  border-color: var(--color-info-border, #60a5fa);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(30, 64, 175, 0.3);
}

.btn.small {
  padding: 6px 12px;
  font-size: 12px;
  height: 32px;
}

/* Header layout */
.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid var(--color-border, #e5e7eb);
  background: var(--color-surface-alt, #f9fafb);
  min-height: 60px;
}

.header-title {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.header-title h2 {
  margin: 0;
  color: var(--color-text-strong, #111827);
  font-size: 1.25rem;
  font-weight: 600;
  line-height: 1.2;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
  flex-wrap: wrap;
}



/* Status badge styles */
.status-badge {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
  height: fit-content;
}

.status-badge.saved {
  background-color: var(--color-success-surface, #d1fae5);
  color: var(--color-success-text, #065f46);
}

.status-badge.unsaved {
  background-color: var(--color-warning-surface, #fef3c7);
  color: var(--color-warning-text, #92400e);
}

.status-badge.initializing {
  background-color: var(--color-info-surface, #dbeafe);
  color: var(--color-info-text, #1e40af);
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  display: inline-block;
}

.status-dot.saved {
  background-color: var(--color-success-bg, #10b981);
}

.status-dot.unsaved {
  background-color: var(--color-warning-bg, #f59e0b);
}

.status-dot.initializing {
  background-color: var(--color-info-bg, #3b82f6);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {

  0%,
  100% {
    opacity: 1;
  }

  50% {
    opacity: 0.5;
  }
}

.status-text {
  font-size: 11px;
  font-weight: 500;
}



@keyframes spin {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}

.btn.loading {
  opacity: 0.7;
  pointer-events: none;
}

/* 浅色主题按钮样式统一 */
.btn.primary {
  background: var(--color-blue-soft-bg, #e3f2fd);
  color: var(--color-blue-soft-text, #1976d2);
  border: 1px solid var(--color-blue-soft-border, #90caf9);
}

.btn.primary:hover {
  background: var(--color-blue-soft-bg, #bbdefb);
  border-color: var(--color-blue-soft-hover, #64b5f6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(25, 118, 210, 0.3);
}

/* 黑暗主题下的按钮样式 */
[data-theme='dark'] .btn.secondary {
  background: rgba(148, 163, 184, 0.2);
  color: #cbd5e1;
  border: 1px solid rgba(148, 163, 184, 0.4);
}

[data-theme='dark'] .btn.secondary:hover {
  background: rgba(148, 163, 184, 0.3);
  border-color: rgba(148, 163, 184, 0.6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(148, 163, 184, 0.4);
}

[data-theme='dark'] .btn.success {
  background: rgba(34, 197, 94, 0.2);
  color: #86efac;
  border: 1px solid rgba(134, 239, 172, 0.4);
}

[data-theme='dark'] .btn.success:hover:not(:disabled) {
  background: rgba(34, 197, 94, 0.3);
  border-color: rgba(110, 231, 183, 0.6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(34, 197, 94, 0.4);
}

[data-theme='dark'] .btn.success:disabled {
  background: rgba(100, 116, 139, 0.2);
  color: rgba(148, 163, 184, 0.6);
  border-color: rgba(100, 116, 139, 0.4);
  cursor: not-allowed;
}

[data-theme='dark'] .btn.warning {
  background: rgba(251, 191, 36, 0.2);
  color: #fde68a;
  border: 1px solid rgba(251, 191, 36, 0.4);
}

[data-theme='dark'] .btn.warning:hover:not(:disabled) {
  background: rgba(251, 191, 36, 0.3);
  border-color: rgba(245, 158, 11, 0.6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(251, 191, 36, 0.4);
}

[data-theme='dark'] .btn.warning:disabled {
  background: rgba(100, 116, 139, 0.2);
  color: rgba(148, 163, 184, 0.6);
  border-color: rgba(100, 116, 139, 0.4);
  cursor: not-allowed;
}

[data-theme='dark'] .btn.info {
  background: rgba(14, 165, 233, 0.2);
  color: #7dd3fc;
  border: 1px solid rgba(125, 211, 252, 0.4);
}

[data-theme='dark'] .btn.info:hover:not(:disabled) {
  background: rgba(14, 165, 233, 0.3);
  border-color: rgba(56, 189, 248, 0.6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(14, 165, 233, 0.4);
}

[data-theme='dark'] .btn.primary {
  background: rgba(59, 130, 246, 0.2);
  color: #93c5fd;
  border: 1px solid rgba(147, 197, 253, 0.4);
}

[data-theme='dark'] .btn.primary:hover {
  background: rgba(59, 130, 246, 0.3);
  border-color: rgba(96, 165, 250, 0.6);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(59, 130, 246, 0.4);
}

[data-theme='dark'] .search-input {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-divider, #374151);
  color: var(--color-text-primary, #f3f4f6);
}

[data-theme='dark'] .search-input:focus {
  border-color: var(--color-primary, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
}

[data-theme='dark'] .clear-search-btn:hover {
  background: var(--color-hover, #374151);
}

[data-theme='dark'] .list-toolbar {
  background: var(--color-surface-alt, #111827);
}

[data-theme='dark'] .open-folder-btn,
[data-theme='dark'] .batch-delete-btn,
[data-theme='dark'] .batch-import-btn,
[data-theme='dark'] .sort-dropdown .sort-btn {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-border, #374151);
  color: var(--color-text-primary, #f9fafb);
}

[data-theme='dark'] .open-folder-btn:hover,
[data-theme='dark'] .batch-import-btn:hover,
[data-theme='dark'] .sort-dropdown .sort-btn:hover {
  background: var(--color-surface-alt, #111827);
  border-color: var(--color-primary, #3b82f6);
}

[data-theme='dark'] .batch-delete-btn:hover:not(:disabled) {
  background: var(--color-surface-alt, #111827);
  border-color: var(--color-danger, #ef4444);
  color: var(--color-danger, #ef4444);
}

[data-theme='dark'] .copy-emails-btn {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-border, #374151);
  color: var(--color-text-primary, #f9fafb);
}

[data-theme='dark'] .copy-emails-btn:hover:not(:disabled) {
  background: var(--color-surface-alt, #111827);
  border-color: var(--color-info, #0ea5e9);
  color: var(--color-info, #0ea5e9);
}

[data-theme='dark'] .pagination-combined {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-border, #374151);
}

[data-theme='dark'] .pagination-info-text {
  color: var(--color-text-primary, #f9fafb);
}

[data-theme='dark'] .pagination-size-select {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-border, #374151);
  color: var(--color-text-primary, #f9fafb);
}

[data-theme='dark'] .pagination-size-select:hover {
  border-color: var(--color-primary, #3b82f6);
}

[data-theme='dark'] .pagination-size-select:focus {
  border-color: var(--color-primary, #3b82f6);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
}

[data-theme='dark'] .page-number {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-border, #374151);
  color: var(--color-text-primary, #f9fafb);
}

[data-theme='dark'] .page-number:hover {
  background: var(--color-surface-alt, #111827);
  border-color: var(--color-primary, #3b82f6);
  color: var(--color-primary, #3b82f6);
}

[data-theme='dark'] .page-number.active {
  background: var(--color-primary, #3b82f6);
  border-color: var(--color-primary, #3b82f6);
  color: #ffffff;
}

[data-theme='dark'] .page-jump-input {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-border, #374151);
  color: var(--color-text-primary, #f9fafb);
}

[data-theme='dark'] .page-jump-input:focus {
  border-color: var(--color-primary, #3b82f6);
}

[data-theme='dark'] .scroll-btn {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-border, #374151);
  color: var(--color-text-primary, #f9fafb);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

[data-theme='dark'] .scroll-btn:hover {
  background: var(--color-primary, #3b82f6);
  border-color: var(--color-primary, #3b82f6);
  color: #ffffff;
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
}

[data-theme='dark'] .sort-menu {
  background: var(--color-surface, #1f2937);
  border-color: var(--color-border, #374151);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
}

[data-theme='dark'] .sort-option {
  color: var(--color-text-primary, #f9fafb);
}

[data-theme='dark'] .sort-option:hover {
  background: var(--color-surface-alt, #111827);
}

[data-theme='dark'] .sort-option.active {
  background: rgba(59, 130, 246, 0.2);
  color: var(--color-primary, #3b82f6);
}

[data-theme='dark'] .sort-divider {
  background: var(--color-border, #374151);
}

/* ==================== 统计概览样式 ==================== */
.stats-overview {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 16px;
  margin-bottom: 20px;
  padding: 20px;
  background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.stat-card {
  background: #ffffff;
  border: 1px solid #e2e8f0;
  border-radius: 10px;
  padding: 20px 16px;
  text-align: center;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  cursor: pointer;
  position: relative;
  overflow: hidden;
}

.stat-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, transparent, currentColor, transparent);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.stat-card:hover::before {
  opacity: 1;
}

.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.08);
  border-color: currentColor;
}

.stat-card.total {
  color: #3b82f6;
}

.stat-card.total:hover {
  background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
}

.stat-card.normal {
  color: #10b981;
}

.stat-card.normal:hover {
  background: linear-gradient(135deg, #f0fdf4 0%, #d1fae5 100%);
}

.stat-card.abnormal {
  color: #ef4444;
}

.stat-card.abnormal:hover {
  background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
}

/* 按额度分类的统计卡片 */
.stat-card.credits-below-4000 {
  color: #f59e0b;
}

.stat-card.credits-below-4000:hover {
  background: linear-gradient(135deg, #fffbeb 0%, #fef3c7 100%);
}

.stat-card.credits-exact-4000 {
  color: #8b5cf6;
}

.stat-card.credits-exact-4000:hover {
  background: linear-gradient(135deg, #faf5ff 0%, #f3e8ff 100%);
}

.stat-card.credits-between-4001-34000 {
  color: #06b6d4;
}

.stat-card.credits-between-4001-34000:hover {
  background: linear-gradient(135deg, #ecfeff 0%, #cffafe 100%);
}

.stat-label {
  font-size: 13px;
  color: #64748b;
  margin-bottom: 12px;
  font-weight: 600;
  letter-spacing: 0.3px;
  text-transform: uppercase;
  opacity: 0.8;
}

.stat-value {
  font-size: 32px;
  font-weight: 700;
  color: currentColor;
  line-height: 1;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

[data-theme='dark'] .stats-overview {
  background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

[data-theme='dark'] .stat-card {
  background: #1e293b;
  border-color: #334155;
}

[data-theme='dark'] .stat-card:hover {
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.4);
  background: #334155;
}

[data-theme='dark'] .stat-label {
  color: #94a3b8;
}

[data-theme='dark'] .stat-value {
  color: currentColor;
}

/* ==================== 过滤按钮样式 ==================== */
.filter-dropdown {
  position: relative;
}

.filter-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: #f3f4f6;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  color: #374151;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.filter-btn:hover {
  background: #e5e7eb;
  border-color: #9ca3af;
}

.filter-btn.active {
  background: #fef3c7;
  border-color: #fbbf24;
  color: #92400e;
}

[data-theme='dark'] .filter-btn {
  background: #374151;
  border-color: #4b5563;
  color: #e5e7eb;
}

[data-theme='dark'] .filter-btn:hover {
  background: #4b5563;
  border-color: #6b7280;
}

[data-theme='dark'] .filter-btn.active {
  background: rgba(251, 191, 36, 0.2);
  border-color: #fbbf24;
  color: #fbbf24;
}

/* 筛选下拉菜单 */
.filter-menu {
  position: absolute;
  top: calc(100% + 8px);
  left: 0;
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  min-width: 180px;
  z-index: 1000;
  overflow: hidden;
}

.filter-menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: #374151;
  font-size: 14px;
}

.filter-menu-item:hover {
  background: #f3f4f6;
}

.filter-menu-item.active {
  background: #fef3c7;
  color: #92400e;
  font-weight: 500;
}

.filter-icon {
  font-size: 16px;
  width: 20px;
  text-align: center;
}

[data-theme='dark'] .filter-menu {
  background: #1f2937;
  border-color: #374151;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

[data-theme='dark'] .filter-menu-item {
  color: #e5e7eb;
}

[data-theme='dark'] .filter-menu-item:hover {
  background: #374151;
}

[data-theme='dark'] .filter-menu-item.active {
  background: rgba(251, 191, 36, 0.2);
  color: #fbbf24;
}

/* ==================== 刷新选项对话框样式 ==================== */
.refresh-options-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  backdrop-filter: blur(4px);
}

.refresh-options-dialog {
  background: white;
  border-radius: 12px;
  width: 90%;
  max-width: 500px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.refresh-options-dialog .dialog-header {
  padding: 20px 24px;
  border-bottom: 1px solid #e5e7eb;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.refresh-options-dialog .dialog-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #111827;
}

.refresh-options-dialog .dialog-body {
  padding: 24px;
}

.dialog-description {
  margin: 0 0 20px 0;
  color: #6b7280;
  font-size: 14px;
}

.refresh-options {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.option-btn {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 8px;
  padding: 16px;
  border: 2px solid #e5e7eb;
  border-radius: 8px;
  background: white;
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
}

.option-btn:hover {
  border-color: #3b82f6;
  background: #f0f9ff;
}

.option-btn svg {
  color: #6b7280;
}

.option-btn span {
  font-weight: 600;
  color: #111827;
  font-size: 14px;
}

.option-desc {
  margin: 0;
  font-size: 12px;
  color: #9ca3af;
}

.option-btn.current-page:hover {
  border-color: #3b82f6;
  background: #f0f9ff;
}

.option-btn.current-page:hover svg {
  color: #3b82f6;
}

.option-btn.all:hover {
  border-color: #10b981;
  background: #f0fdf4;
}

.option-btn.all:hover svg {
  color: #10b981;
}

[data-theme='dark'] .refresh-options-dialog {
  background: #1f2937;
}

[data-theme='dark'] .refresh-options-dialog .dialog-header {
  border-bottom-color: #374151;
}

[data-theme='dark'] .refresh-options-dialog .dialog-header h3 {
  color: #f9fafb;
}

[data-theme='dark'] .dialog-description {
  color: #d1d5db;
}

[data-theme='dark'] .option-btn {
  background: #111827;
  border-color: #374151;
}

[data-theme='dark'] .option-btn:hover {
  background: #1f2937;
  border-color: #3b82f6;
}

[data-theme='dark'] .option-btn span {
  color: #f9fafb;
}

[data-theme='dark'] .option-desc {
  color: #9ca3af;
}

/* ==================== 去重模态框样式 ==================== */
.duplicate-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  backdrop-filter: blur(4px);
}

.duplicate-modal-content {
  background: white;
  border-radius: 12px;
  width: 90%;
  max-width: 800px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.duplicate-modal-header {
  padding: 20px 24px;
  border-bottom: 1px solid #e5e7eb;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.duplicate-modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: #111827;
}

.duplicate-modal-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.duplicate-stats-card {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.stat-item {
  background: #f9fafb;
  padding: 16px;
  border-radius: 8px;
  text-align: center;
  border: 2px solid #e5e7eb;
}

.stat-item.warning {
  background: #fef3c7;
  border-color: #fbbf24;
}

.stat-item.danger {
  background: #fee2e2;
  border-color: #ef4444;
}

.stat-label {
  font-size: 13px;
  color: #6b7280;
  margin-bottom: 8px;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: #111827;
}

.duplicate-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.duplicate-group {
  background: #f9fafb;
  border-radius: 8px;
  padding: 16px;
  border: 1px solid #e5e7eb;
}

.duplicate-group-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
  padding-bottom: 12px;
  border-bottom: 1px solid #e5e7eb;
}

.email-text {
  font-weight: 600;
  color: #111827;
  font-size: 14px;
}

.count-badge {
  background: #fbbf24;
  color: #78350f;
  padding: 4px 12px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
}

.duplicate-tokens {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.duplicate-token-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  border-radius: 6px;
  background: white;
  border: 1px solid #e5e7eb;
}

.duplicate-token-item.keep {
  border-color: #10b981;
  background: #f0fdf4;
}

.duplicate-token-item.delete {
  border-color: #ef4444;
  background: #fef2f2;
}

.token-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.token-id {
  font-family: monospace;
  font-size: 12px;
  color: #6b7280;
}

.token-date {
  font-size: 11px;
  color: #9ca3af;
}

.token-action {
  font-weight: 600;
  font-size: 13px;
  padding: 4px 12px;
  border-radius: 4px;
}

.token-action.keep {
  color: #10b981;
  background: #d1fae5;
}

.token-action.delete {
  color: #ef4444;
  background: #fee2e2;
}

.duplicate-modal-footer {
  padding: 16px 24px;
  border-top: 1px solid #e5e7eb;
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

/* 深色主题 */
[data-theme='dark'] .duplicate-modal-content {
  background: #1f2937;
}

[data-theme='dark'] .duplicate-modal-header {
  border-bottom-color: #374151;
}

[data-theme='dark'] .duplicate-modal-header h3 {
  color: #f9fafb;
}

[data-theme='dark'] .stat-item {
  background: #111827;
  border-color: #374151;
}

[data-theme='dark'] .stat-item.warning {
  background: rgba(251, 191, 36, 0.1);
  border-color: #fbbf24;
}

[data-theme='dark'] .stat-item.danger {
  background: rgba(239, 68, 68, 0.1);
  border-color: #ef4444;
}

[data-theme='dark'] .stat-label {
  color: #9ca3af;
}

[data-theme='dark'] .stat-value {
  color: #f9fafb;
}

[data-theme='dark'] .duplicate-group {
  background: #111827;
  border-color: #374151;
}

[data-theme='dark'] .duplicate-group-header {
  border-bottom-color: #374151;
}

[data-theme='dark'] .email-text {
  color: #f9fafb;
}

[data-theme='dark'] .duplicate-token-item {
  background: #1f2937;
  border-color: #374151;
}

[data-theme='dark'] .duplicate-token-item.keep {
  border-color: #10b981;
  background: rgba(16, 185, 129, 0.1);
}

[data-theme='dark'] .duplicate-token-item.delete {
  border-color: #ef4444;
  background: rgba(239, 68, 68, 0.1);
}

[data-theme='dark'] .token-id {
  color: #9ca3af;
}

[data-theme='dark'] .token-date {
  color: #6b7280;
}

[data-theme='dark'] .duplicate-modal-footer {
  border-top-color: #374151;
}
</style>
