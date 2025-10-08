<template>
  <div class="container">
    <header>
      <h1>{{ t('app_title') }}</h1>
      <div class="language-selector">
        <button @click="changeLanguage('en')">English</button>
        <button @click="changeLanguage('zh')">中文</button>
      </div>
    </header>

    <!-- 使用说明 -->
    <div class="instructions" v-if="accounts.length === 0">
      <h3>{{ t('usage_title') }}</h3>
      <ol>
        <li>{{ t('usage_step1') }}</li>
        <li>{{ t('usage_step2') }}</li>
        <li>{{ t('usage_step3') }}</li>
        <li>{{ t('usage_step4') }}</li>
      </ol>
      <div class="how-to-get-json">
        <h4>{{ t('how_to_get_account') }}</h4>
        <p>{{ t('get_account_desc') }}</p>
        <code>{{ t('get_account_example') }}</code>
      </div>
    </div>

    <div class="toolbar">
      <button @click="showAddDialog = true" class="btn-add">
        {{ t('add_account') }}
      </button>
      <button @click="showInstructions = !showInstructions" class="btn-help">
        {{ t('help') }}
      </button>
      <button 
        v-if="isProxyRunning" 
        @click="handleStopProxy" 
        class="btn-stop" 
        :disabled="loading"
      >
        {{ t('stop_proxy') }}
      </button>
      <button 
        v-else 
        @click="handleStartProxy" 
        class="btn-start" 
        :disabled="loading"
      >
        {{ t('start_proxy') }}
      </button>
      <div class="proxy-status">
        {{ t('proxy_status') }}: {{ isProxyRunning ? t('running') : t('stopped') }}
      </div>
    </div>

    <!-- 帮助说明弹窗 -->
    <div v-if="showInstructions" class="dialog-overlay" @click="showInstructions = false">
      <div class="dialog instructions-dialog" @click.stop>
        <h2>{{ t('usage_title') }}</h2>
        <ol>
          <li v-html="t('usage_step1')"></li>
          <li v-html="t('usage_step2')"></li>
          <li v-html="t('usage_step3')"></li>
          <li v-html="t('usage_step4')"></li>
        </ol>
        <div class="how-to-get-json">
          <h4>{{ t('how_to_get_account') }}</h4>
          <p v-html="t('get_account_desc')"></p>
          <pre><code>{{ t('get_account_example') }}</code></pre>
        </div>
        <button @click="showInstructions = false" class="btn-primary">
          {{ t('got_it') }}
        </button>
      </div>
    </div>

    <div v-if="activeAccount" class="active-account">
      {{ t('active_account') }}: <strong>{{ activeAccount }}</strong>
    </div>

    <div class="accounts-table">
      <table>
        <thead>
          <tr>
            <th>{{ t('email') }}</th>
            <th>{{ t('health_status') }}</th>
            <th>{{ t('limit_info') }}</th>
            <th>{{ t('last_updated') }}</th>
            <th>{{ t('actions') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr 
            v-for="account in accounts" 
            :key="account.id"
            :class="{ 'active-row': activeAccount === account.email }"
          >
            <td>{{ account.email }}</td>
            <td>
              <span :class="`status-${account.health_status}`">
                {{ t(account.health_status) }}
              </span>
            </td>
            <td>{{ account.limit_info || 'N/A' }}</td>
            <td>{{ formatDate(account.last_updated) }}</td>
            <td>
              <div class="action-buttons">
                <button 
                  @click="handleSetActive(account.email)"
                  :disabled="activeAccount === account.email"
                  class="btn-icon"
                  :title="t('set_active')"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="20 6 9 17 4 12"></polyline>
                  </svg>
                </button>
                <button 
                  @click="handleGetLimitInfo(account.email)"
                  class="btn-icon"
                  :title="t('get_limits')"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                    <polyline points="7 10 12 15 17 10"></polyline>
                    <line x1="12" y1="15" x2="12" y2="3"></line>
                  </svg>
                </button>
                <button 
                  @click="handleDeleteAccount(account.email)"
                  class="btn-icon btn-icon-danger"
                  :title="t('delete')"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="3 6 5 6 21 6"></polyline>
                    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                    <line x1="10" y1="11" x2="10" y2="17"></line>
                    <line x1="14" y1="11" x2="14" y2="17"></line>
                  </svg>
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- 添加账户对话框 -->
    <div v-if="showAddDialog" class="dialog-overlay" @click="showAddDialog = false">
      <div class="dialog" @click.stop>
        <h2>{{ t('add_account_title') }}</h2>
        <p>{{ t('add_account_instruction') }}</p>
        <textarea
          v-model="accountJson"
          :placeholder="t('add_account_placeholder')"
          rows="10"
        />
        <div class="dialog-buttons">
          <button @click="handleAddAccount" :disabled="loading" class="btn-primary">
            {{ t('add') }}
          </button>
          <button @click="showAddDialog = false" class="btn-secondary">
            {{ t('cancel') }}
          </button>
        </div>
      </div>
    </div>

    <!-- 加载动画 -->
    <div v-if="loading" class="loading-overlay">
      <div class="spinner"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { useI18n } from 'vue-i18n';

const { t, locale } = useI18n();

interface Account {
  id: number;
  email: string;
  account_data: string;
  health_status: string;
  limit_info: string | null;
  last_updated: string;
}

const accounts = ref<Account[]>([]);
const activeAccount = ref<string | null>(null);
const isProxyRunning = ref(false);
const showAddDialog = ref(false);
const showInstructions = ref(false);
const accountJson = ref('');
const loading = ref(false);

onMounted(() => {
  loadAccounts();
  loadActiveAccount();
  checkProxyStatus();
});

const loadAccounts = async () => {
  try {
    const accts = await invoke<Account[]>('get_accounts');
    accounts.value = accts;
  } catch (error) {
    console.error('Failed to load accounts:', error);
  }
};

const loadActiveAccount = async () => {
  try {
    const active = await invoke<string | null>('get_active_account');
    activeAccount.value = active;
  } catch (error) {
    console.error('Failed to load active account:', error);
  }
};

const checkProxyStatus = async () => {
  try {
    const running = await invoke<boolean>('is_proxy_running');
    isProxyRunning.value = running;
  } catch (error) {
    console.error('Failed to check proxy status:', error);
  }
};

const handleAddAccount = async () => {
  if (!accountJson.value.trim()) return;
  
  loading.value = true;
  try {
    await invoke('add_account', { accountJson: accountJson.value });
    accountJson.value = '';
    showAddDialog.value = false;
    await loadAccounts();
    alert(t('account_added_success'));
  } catch (error) {
    alert(`${t('error')}: ${error}`);
  } finally {
    loading.value = false;
  }
};

const handleDeleteAccount = async (email: string) => {
  if (!confirm(t('confirm_delete', { email }))) return;
  
  try {
    await invoke('delete_account', { email });
    await loadAccounts();
    if (activeAccount.value === email) {
      activeAccount.value = null;
    }
  } catch (error) {
    alert(`${t('error')}: ${error}`);
  }
};

const handleSetActive = async (email: string) => {
  try {
    await invoke('set_active_account', { email });
    activeAccount.value = email;
    alert(t('active_account_set', { email }));
  } catch (error) {
    alert(`${t('error')}: ${error}`);
  }
};

const handleGetLimitInfo = async (email: string) => {
  loading.value = true;
  try {
    const limitInfo = await invoke<string>('get_limit_info', { email });
    await invoke('update_account_limit_info', { email, limitInfo });
    await loadAccounts();
  } catch (error) {
    alert(`${t('error')}: ${error}`);
  } finally {
    loading.value = false;
  }
};

const handleStartProxy = async () => {
  if (!activeAccount.value) {
    alert(t('no_active_account'));
    return;
  }

  loading.value = true;
  try {
    await invoke('start_proxy');
    isProxyRunning.value = true;
    alert(t('proxy_started'));
  } catch (error) {
    alert(`${t('error')}: ${error}`);
  } finally {
    loading.value = false;
  }
};

const handleStopProxy = async () => {
  loading.value = true;
  try {
    await invoke('stop_proxy');
    isProxyRunning.value = false;
    alert(t('proxy_stopped'));
  } catch (error) {
    alert(`${t('error')}: ${error}`);
  } finally {
    loading.value = false;
  }
};

const changeLanguage = (lang: string) => {
  locale.value = lang;
};

const formatDate = (dateStr: string) => {
  return new Date(dateStr).toLocaleString();
};
</script>

<style scoped>
@import './App.css';
</style>
