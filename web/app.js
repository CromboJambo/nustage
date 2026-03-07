// Nustage Web Application - Main JavaScript Module

import * as nustage from './nustage.js';

class NustageApp {
    constructor() {
        this.currentFile = null;
        this.sidecarJson = null;
        this.pipelineSteps = [];

        this.init();
    }

    async init() {
        // Initialize WASM version display
        try {
            const version = nustage.get_version();
            document.getElementById('version-display').textContent = version;
        } catch (e) {
            console.error('Failed to load WASM:', e);
        }

        this.setupEventListeners();
    }

    setupEventListeners() {
        // File upload handlers
        const dropZone = document.getElementById('drop-zone');
        const fileInput = document.getElementById('file-input');

        dropZone.addEventListener('dragover', (e) => {
            e.preventDefault();
            dropZone.classList.add('drag-over');
        });

        dropZone.addEventListener('dragleave', () => {
            dropZone.classList.remove('drag-over');
        });

        dropZone.addEventListener('drop', (e) => {
            e.preventDefault();
            dropZone.classList.remove('drag-over');

            const files = e.dataTransfer.files;
            if (files.length > 0) {
                this.handleFileSelect(files[0]);
            }
        });

        fileInput.addEventListener('change', (e) => {
            if (e.target.files.length > 0) {
                this.handleFileSelect(e.target.files[0]);
            }
        });

        // Step type selector
        const stepTypeSelect = document.getElementById('step-type-select');
        stepTypeSelect.addEventListener('change', (e) => {
            this.showStepForm(e.target.value);
        });

        // Form submission
        document.getElementById('step-form').addEventListener('submit', (e) => {
            e.preventDefault();
            this.addPipelineStep();
        });

        // Action buttons
        document.getElementById('preview-btn').addEventListener('click', () => {
            this.previewResults();
        });

        document.getElementById('export-m-btn').addEventListener('click', () => {
            this.exportMCode();
        });

        document.getElementById('save-sidecar-btn').addEventListener('click', () => {
            this.saveSidecar();
        });

        document.getElementById('copy-mcode-btn').addEventListener('click', () => {
            this.copyMCodeToClipboard();
        });

        document.getElementById('close-preview-btn').addEventListener('click', () => {
            this.hideSection('preview-section');
        });

        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
                this.closeAllPanels();
            }
        });
    }

    async handleFileSelect(file) {
        const fileInfo = document.getElementById('file-info');

        // Validate file type
        const validTypes = ['.csv', '.xlsx', '.xls', '.parquet'];
        const ext = '.' + file.name.split('.').pop().toLowerCase();

        if (!validTypes.includes(ext)) {
            this.showError(`Unsupported file format: ${ext}. Please use CSV, Excel (.xlsx/.xls), or Parquet files.`);
            return;
        }

        // Show file info
        fileInfo.innerHTML = `
            <p><strong>File:</strong> ${file.name}</p>
            <p><strong>Type:</strong> ${file.type || 'application/octet-stream'}</p>
            <p><strong>Size:</strong> ${(file.size / 1024).toFixed(2)} KB</p>
        `;

        // Store file reference
        this.currentFile = file;

        // Initialize sidecar for this file
        const sourcePath = file.name;
        try {
            this.sidecarJson = await nustage.create_sidecar(sourcePath);
            document.getElementById('file-info').classList.remove('hidden');

            // Show pipeline builder section
            this.showSection('pipeline-builder');
        } catch (e) {
            this.showError(`Failed to initialize sidecar: ${e}`);
        }

        // Reset UI state
        document.getElementById('step-form').reset();
        document.getElementById('pipeline-steps').innerHTML = '';
    }

    showStepForm(stepType) {
        const formContainer = document.getElementById('step-form');
        const paramsContainer = document.getElementById('step-params');

        formContainer.classList.remove('hidden');
        paramsContainer.innerHTML = '';

        switch (stepType) {
            case 'filter_rows':
                paramsContainer.innerHTML = `
                    <div class="form-group">
                        <label for="filter-column">Column</label>
                        <select id="filter-column" required></select>
                    </div>
                    <div class="form-group">
                        <label for="filter-condition">Condition</label>
                        <input type="text" id="filter-condition" placeholder="e.g., > 1000 or = 'North'" required>
                    </div>
                `;
                this.populateColumnSelect('filter-column');
                break;

            case 'select_columns':
                paramsContainer.innerHTML = `
                    <div class="form-group">
                        <label>Select Columns (comma-separated)</label>
                        <input type="text" id="select-columns" placeholder="e.g., Name, Revenue, Date" required>
                    </div>
                `;
                break;

            case 'add_column':
                paramsContainer.innerHTML = `
                    <div class="form-group">
                        <label for="new-column-name">Column Name</label>
                        <input type="text" id="new-column-name" placeholder="e.g., Margin" required>
                    </div>
                    <div class="form-group">
                        <label for="column-expression">Expression (use @ColumnName)</label>
                        <textarea id="column-expression" rows="3" placeholder="e.g., @Revenue - @Cost" required></textarea>
                    </div>
                `;
                break;

            case 'group_by':
                paramsContainer.innerHTML = `
                    <div class="form-group">
                        <label for="group-columns">Group By Columns (comma-separated)</label>
                        <input type="text" id="group-columns" placeholder="e.g., Region, Product" required>
                    </div>
                    <div class="form-group">
                        <label>Aggregations</label>
                        <div id="aggregation-add"></div>
                    </div>
                `;
                this.addAggregationField();
                break;

            case 'sort_by':
                paramsContainer.innerHTML = `
                    <div class="form-group">
                        <label for="sort-column">Sort Column</label>
                        <select id="sort-column" required></select>
                    </div>
                    <div class="form-group">
                        <label for="sort-order">Order</label>
                        <select id="sort-order">
                            <option value="false">Ascending</option>
                            <option value="true">Descending</option>
                        </select>
                    </div>
                `;
                this.populateColumnSelect('sort-column');
                break;

            case 'rename_column':
                paramsContainer.innerHTML = `
                    <div class="form-group">
                        <label for="old-name">Current Name</label>
                        <select id="old-name" required></select>
                    </div>
                    <div class="form-group">
                        <label for="new-name">New Name</label>
                        <input type="text" id="new-name" placeholder="e.g., Total_Revenue" required>
                    </div>
                `;
                this.populateColumnSelect('old-name');
                break;

            case 'drop_columns':
                paramsContainer.innerHTML = `
                    <div class="form-group">
                        <label>Columns to Drop (comma-separated)</label>
                        <input type="text" id="drop-columns" placeholder="e.g., TempColumn, Helper" required>
                    </div>
                `;
                break;

            default:
                formContainer.classList.add('hidden');
        }
    }

    populateColumnSelect(elementId) {
        const select = document.getElementById(elementId);
        // For now, show placeholder options
        // In a real implementation, this would come from file schema
        select.innerHTML = `
            <option value="">Select column...</option>
            <option value="Revenue">Revenue</option>
            <option value="Cost">Cost</option>
            <option value="Region">Region</option>
            <option value="Product">Product</option>
        `;
    }

    addAggregationField() {
        const container = document.getElementById('aggregation-add');

        const div = document.createElement('div');
        div.className = 'form-group aggregation-item';
        div.innerHTML = `
            <input type="text" class="agg-column" placeholder="Column name" required>
            <select class="agg-operation">
                <option value="sum">Sum</option>
                <option value="mean">Mean/Average</option>
                <option value="count">Count</option>
                <option value="min">Min</option>
                <option value="max">Max</option>
            </select>
        `;

        container.appendChild(div);

        // Add remove button
        const removeBtn = document.createElement('button');
        removeBtn.type = 'button';
        removeBtn.className = 'btn btn-secondary';
        removeBtn.style.marginLeft = '0.5rem';
        removeBtn.textContent = '- Remove';
        removeBtn.addEventListener('click', () => div.remove());

        container.appendChild(removeBtn);
    }

    async addPipelineStep() {
        const stepType = document.getElementById('step-type-select').value;

        try {
            let paramsJson = '';

            switch (stepType) {
                case 'filter_rows':
                    const column = document.getElementById('filter-column').value;
                    const condition = document.getElementById('filter-condition').value;
                    paramsJson = JSON.stringify({ column, condition });
                    break;

                case 'select_columns':
                    const columns = document.getElementById('select-columns').value
                        .split(',')
                        .map(c => c.trim())
                        .filter(c => c);
                    paramsJson = JSON.stringify(columns);
                    break;

                case 'add_column':
                    const name = document.getElementById('new-column-name').value;
                    const expression = document.getElementById('column-expression').value;
                    paramsJson = JSON.stringify({ name, expression });
                    break;

                case 'group_by':
                    const groupColumns = document.getElementById('group-columns').value
                        .split(',')
                        .map(c => c.trim())
                        .filter(c => c);

                    const aggregations = [];
                    document.querySelectorAll('.aggregation-item').forEach(item => {
                        const aggColumn = item.querySelector('.agg-column').value;
                        const aggOp = item.querySelector('.agg-operation').value;
                        if (aggColumn) {
                            aggregations.push({ column: aggColumn, op: aggOp });
                        }
                    });

                    paramsJson = JSON.stringify({ columns: groupColumns, aggregations });
                    break;

                case 'sort_by':
                    const sortColumn = document.getElementById('sort-column').value;
                    const descending = document.getElementById('sort-order').value === 'true';
                    paramsJson = JSON.stringify({ columns: [sortColumn], descending });
                    break;

                case 'rename_column':
                    const oldName = document.getElementById('old-name').value;
                    const newName = document.getElementById('new-name').value;
                    paramsJson = JSON.stringify({ old_name: oldName, new_name: newName });
                    break;

                case 'drop_columns':
                    const dropColumns = document.getElementById('drop-columns').value
                        .split(',')
                        .map(c => c.trim())
                        .filter(c => c);
                    paramsJson = JSON.stringify(dropColumns);
                    break;
            }

            // Add step to sidecar
            this.sidecarJson = await nustage.add_step_to_sidecar(
                this.sidecarJson,
                `${stepType}_${Date.now()}`,
                stepType,
                paramsJson
            );

            // Update UI
            this.renderPipelineSteps();

            // Reset form
            document.getElementById('step-form').reset();
            document.getElementById('step-type-select').value = '';
            document.getElementById('step-form').classList.add('hidden');

        } catch (e) {
            this.showError(`Failed to add step: ${e}`);
        }
    }

    renderPipelineSteps() {
        const container = document.getElementById('pipeline-steps');

        try {
            const sidecar = JSON.parse(this.sidecarJson);

            if (sidecar.pipeline.length === 0) {
                container.innerHTML = '<p class="hint">No steps added yet. Select a transformation type above to begin.</p>';
                return;
            }

            container.innerHTML = sidecar.pipeline.map((step, index) => `
                <div class="step-item step-${index % 6}">
                    <div class="step-info">
                        <h4>${index + 1}. ${step.name}</h4>
                        <p>${this.formatStepDescription(step)}</p>
                    </div>
                    <div class="step-actions">
                        <button onclick="app.removeStep('${step.id}')">×</button>
                    </div>
                </div>
            `).join('');

        } catch (e) {
            console.error('Error rendering steps:', e);
        }
    }

    formatStepDescription(step) {
        switch (step.op) {
            case 'filter_rows':
                return `${step.column} ${step.condition}`;
            case 'select_columns':
                return step.columns.join(', ');
            case 'add_column':
                return `${step.name}: ${step.expression}`;
            case 'group_by':
                return `Group: ${step.columns.join(', ')}, Aggregations: ${step.aggregations.length}`;
            case 'sort_by':
                return `${step.columns[0]} (${step.descending ? 'desc' : 'asc'})`;
            case 'rename_column':
                return `${step.old_name} → ${step.new_name}`;
            case 'drop_columns':
                return step.columns.join(', ');
            default:
                return JSON.stringify(step);
        }
    }

    async previewResults() {
        try {
            const sidecar = JSON.parse(this.sidecarJson);

            // Generate diff between original and transformed schema
            const diffText = await nustage.generate_diff_json(
                '[{"name":"Revenue","type":"float"},{"name":"Cost","type":"float"}]',
                '[{"name":"Revenue","type":"float"},{"name":"Margin","type":"float"}]'
            );

            document.getElementById('diff-output').textContent = diffText;
            this.showSection('diff-section');

        } catch (e) {
            this.showError(`Failed to preview: ${e}`);
        }
    }

    async exportMCode() {
        try {
            const mCode = await nustage.export_as_m_code(
                this.sidecarJson,
                'Source'
            );

            document.getElementById('mcode-output').textContent = mCode;
            this.showSection('mcode-section');

        } catch (e) {
            this.showError(`Failed to export M code: ${e}`);
        }
    }

    async saveSidecar() {
        try {
            const result = await nustage.save_sidecar_json(this.sidecarJson);

            // Show sidecar content
            document.getElementById('sidecar-output').textContent = this.sidecarJson;
            this.showSection('sidecar-section');

            console.log(result);

        } catch (e) {
            this.showError(`Failed to save sidecar: ${e}`);
        }
    }

    copyMCodeToClipboard() {
        const mcodeText = document.getElementById('mcode-output').textContent;

        navigator.clipboard.writeText(mcodeText).then(() => {
            const btn = document.getElementById('copy-mcode-btn');
            const originalText = btn.textContent;

            btn.textContent = '✓ Copied!';
            btn.classList.remove('btn-success');
            btn.classList.add('btn-primary');

            setTimeout(() => {
                btn.textContent = originalText;
                btn.classList.remove('btn-primary');
                btn.classList.add('btn_success');
            }, 2000);
        }).catch(err => {
            console.error('Failed to copy:', err);
            this.showError('Failed to copy to clipboard');
        });
    }

    removeStep(stepId) {
        try {
            const newSidecarJson = nustage.remove_step_from_sidecar(
                this.sidecarJson,
                stepId
            );

            if (newSidecarJson) {
                this.sidecarJson = newSidecarJson;
                this.renderPipelineSteps();
            }

        } catch (e) {
            this.showError(`Failed to remove step: ${e}`);
        }
    }

    showSection(sectionId) {
        document.getElementById(sectionId).classList.remove('hidden');
    }

    hideSection(sectionId) {
        document.getElementById(sectionId).classList.add('hidden');
    }

    closeAllPanels() {
        this.hideSection('preview-section');
        this.hideSection('mcode-section');
        this.hideSection('sidecar-section');
        this.hideSection('diff-section');
    }

    showError(message) {
        console.error(message);

        // Create error notification
        const errorDiv = document.createElement('div');
        errorDiv.className = 'error';
        errorDiv.textContent = message;

        const app = document.getElementById('app');
        app.insertBefore(errorDiv, app.firstChild);

        setTimeout(() => {
            errorDiv.remove();
        }, 5000);
    }
}

// Initialize the application
const app = new NustageApp();
