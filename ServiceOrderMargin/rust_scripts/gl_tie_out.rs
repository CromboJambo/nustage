use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// Define structs for the data structures
#[derive(Debug, Clone)]
struct GLPostedTransaction {
    date: String,
    sequence: i32,
    name: String,
    reference: String,
    posted_from: String,
    account_description: String,
    account: String,
    account_unit_1: String,
    account_unit_2: String,
    account_unit_3: String,
    account_unit_4: String,
    debit_domestic: f64,
    credit_domestic: f64,
    exchange_rate: f64,
    debit_foreign: f64,
    credit_foreign: f64,
    allocation_transaction: String,
    site: String,
    posted_from_site: String,
    gl_voucher: String,
    approver: String,
    cust_vendor: String,
    invoice_voucher: String,
    cancellation: String,
    invoice_vch_seq: String,
    hierarchy: String,
    control_prefix: String,
    control_site: String,
    control_year: i32,
    control_period: i32,
    control_number: i32,
    ref_control_prefix: String,
    ref_control_site: String,
    ref_control_year: i32,
    ref_control_period: i32,
    ref_control_number: i32,
    document_number: String,
    check_number: String,
    check_date: String,
    currency: String,
    bank_code: String,
    consolidated: String,
    vendor_number_source: String,
    analysis_attribute_1: String,
    analysis_attribute_2: String,
    analysis_attribute_3: String,
    analysis_attribute_4: String,
    analysis_attribute_5: String,
    analysis_attribute_6: String,
    analysis_attribute_7: String,
    analysis_attribute_8: String,
    analysis_attribute_9: String,
    analysis_attribute_10: String,
    analysis_attribute_11: String,
    analysis_attribute_12: String,
    analysis_attribute_13: String,
    analysis_attribute_14: String,
    analysis_attribute_15: String,
}

#[derive(Debug, Clone)]
struct ServiceOrderTransaction {
    sro: String,
    trans_date: String,
    post_date: String,
    posted: i32,
    bill_status: String,
    invoice: String,
    partner: String,
    work_code: String,
    ext_cost: f64,
    hours_worked: f64,
    hours_to_bill: f64,
    cost: f64,
    ext_price: f64,
    work_code_description: String,
    sro_description: String,
    billing_code: String,
    variable_overhead_cost: f64,
    labor_rate: f64,
    reimbursement_method: String,
    fixed_overhead_cost: f64,
    start_time: String,
    end_time: String,
    line_description: String,
    oper_description: String,
    trans_type: String,
    trans_num: String,
    line_number: i32,
    oper_number: i32,
    description: String,
    reimbursement_tax_code_1: String,
    payroll: String,
    reimburse_labor: f64,
    discount: f64,
    reimbursement_tax: f64,
    sign_off_date: String,
    tax_code: String,
    sign_off: String,
    cost_currency: String,
    price_currency: String,
    reimbursement_currency: String,
    dept: String,
    warehouse: String,
    amount_format_1: String,
    amount_format_2: String,
    bill_hold: i32,
}

#[derive(Debug, Clone)]
struct MaterialTransaction {
    sro: String,
    sro_description: String,
    item: String,
    item_description: String,
    unit: String,
    customer: String,
    exchange_date: String,
    trans_date: String,
    post_date: String,
    billing_code: String,
    packing_slip: String,
    posted: i32,
    bill_status: String,
    invoice: String,
    quantity: f64,
    ext_cost: f64,
    oper_description: String,
    line_description: String,
    matl_cost: f64,
    labor_cost: f64,
    vovhd_cost: f64,
    unit_cost: f64,
    location: String,
    line_number: i32,
    trans_num: String,
    oper_number: i32,
    discount: f64,
    impact_inventory: i32,
    item_exists: i32,
    revision: String,
    unit_price: f64,
    ext_price: f64,
    um: String,
    dept: String,
    partner: String,
    drop_ship_to: String,
    cost_type: String,
    trans_type: String,
    tax_code: String,
    outside_cost: f64,
    qty_back_ordered: f64,
    reimbursement_tax: f64,
    fovhd_cost: f64,
    reason_code: String,
    document_number: String,
    transaction_restriction_code: String,
    customer_item: String,
    lot: String,
    reimbursement_tax_code_1: String,
    reimbursement_currency: String,
    price_code: String,
    sign_off: String,
    sign_off_date: String,
    notc: String,
    notc_description: String,
    delivery_terms: String,
    delivery_terms_description: String,
    origin: String,
    eu_code: String,
    commodity: String,
    process_indicator: String,
    supply_units: f64,
    usage_type: String,
    address: String,
    reimbursement_method: String,
    s_n_prefix: String,
    lot_prefix: String,
    reimburse_material: f64,
    lot_tracked: i32,
    serial_tracked: i32,
    bill_hold: i32,
}

#[derive(Debug, Clone)]
struct MiscTransaction {
    sro: String,
    trans_date: String,
    post_date: String,
    posted: i32,
    bill_status: String,
    invoice: String,
    billing_code: String,
    misc_code: String,
    misc_code_description: String,
    quantity: f64,
    ext_cost: f64,
    ext_price: f64,
    matl_cost: f64,
    reimbursement_total_due: f64,
    oper_description: String,
    sro_description: String,
    line_description: String,
    invoice_1: String,
    unit_cost: f64,
    partner: String,
    payment_type: String,
    labor_cost: f64,
    vovhd_cost: f64,
    discount: f64,
    unit_price: f64,
    misc_pay_type_reimbursable_grid_col: i32,
    tax_code: String,
    reimbursement_tax_code_1: String,
    bill_hold: i32,
    cost_currency: String,
    reimbursement_tax: f64,
    outside_cost: f64,
    fovhd_cost: f64,
    line_number: i32,
    oper_number: i32,
    trans_num: String,
    trans_type: String,
    dept: String,
    sign_off: String,
    sign_off_date: String,
    warehouse: String,
    reimbursement_status: String,
    reimbursement_currency: String,
    price_currency: String,
    amount_format_1: String,
    amount_format_2: String,
    amount_format_3: String,
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in line.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                current.push(c);
            }
            ',' if !in_quotes => {
                result.push(current.clone());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    result.push(current);
    result
}

fn parse_float(s: &str) -> Option<f64> {
    if s.trim().is_empty() {
        return None;
    }
    let cleaned = s.trim().replace(',', "");
    cleaned.parse().ok()
}

fn parse_int(s: &str) -> Option<i32> {
    if s.trim().is_empty() {
        return None;
    }
    s.trim().parse().ok()
}

fn read_gl_transactions(path: &str) -> Result<Vec<GLPostedTransaction>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut transactions = Vec::new();
    let mut headers: Option<Vec<String>> = None;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let fields = parse_csv_line(&line);

        if headers.is_none() {
            headers = Some(fields);
            continue;
        }

        if let Some(headers) = &headers {
            if let Some(gl_trans) = parse_gl_transaction(&fields, headers) {
                transactions.push(gl_trans);
            }
        }
    }

    Ok(transactions)
}

fn parse_gl_transaction(fields: &[String], headers: &[String]) -> Option<GLPostedTransaction> {
    let mut get_field = |index: usize| -> Option<&String> {
        if index < headers.len() {
            Some(&fields[index])
        } else {
            None
        }
    };

    let get_float = |index: usize| -> Option<f64> {
        get_field(index).and_then(|s| parse_float(s))
    };

    let get_int = |index: usize| -> Option<i32> {
        get_field(index).and_then(|s| parse_int(s))
    };

    Some(GLPostedTransaction {
        date: get_field(0)?.clone(),
        sequence: get_int(1)?,
        name: get_field(2)?.clone(),
        reference: get_field(3)?.clone(),
        posted_from: get_field(4)?.clone(),
        account_description: get_field(5)?.clone(),
        account: get_field(6)?.clone(),
        account_unit_1: get_field(7)?.clone(),
        account_unit_2: get_field(8)?.clone(),
        account_unit_3: get_field(9)?.clone(),
        account_unit_4: get_field(10)?.clone(),
        debit_domestic: get_float(11)?,
        credit_domestic: get_float(12)?,
        exchange_rate: get_float(13)?,
        debit_foreign: get_float(14)?,
        credit_foreign: get_float(15)?,
        allocation_transaction: get_field(16)?.clone(),
        site: get_field(17)?.clone(),
        posted_from_site: get_field(18)?.clone(),
        gl_voucher: get_field(19)?.clone(),
        approver: get_field(20)?.clone(),
        cust_vendor: get_field(21)?.clone(),
        invoice_voucher: get_field(22)?.clone(),
        cancellation: get_field(23)?.clone(),
        invoice_vch_seq: get_field(24)?.clone(),
        hierarchy: get_field(25)?.clone(),
        control_prefix: get_field(26)?.clone(),
        control_site: get_field(27)?.clone(),
        control_year: get_int(28)?,
        control_period: get_int(29)?,
        control_number: get_int(30)?,
        ref_control_prefix: get_field(31)?.clone(),
        ref_control_site: get_field(32)?.clone(),
        ref_control_year: get_int(33)?,
        ref_control_period: get_int(34)?,
        ref_control_number: get_int(35)?,
        document_number: get_field(36)?.clone(),
        check_number: get_field(37)?.clone(),
        check_date: get_field(38)?.clone(),
        currency: get_field(39)?.clone(),
        bank_code: get_field(40)?.clone(),
        consolidated: get_field(41)?.clone(),
        vendor_number_source: get_field(42)?.clone(),
        analysis_attribute_1: get_field(43)?.clone(),
        analysis_attribute_2: get_field(44)?.clone(),
        analysis_attribute_3: get_field(45)?.clone(),
        analysis_attribute_4: get_field(46)?.clone(),
        analysis_attribute_5: get_field(47)?.clone(),
        analysis_attribute_6: get_field(48)?.clone(),
        analysis_attribute_7: get_field(49)?.clone(),
        analysis_attribute_8: get_field(50)?.clone(),
        analysis_attribute_9: get_field(51)?.clone(),
        analysis_attribute_10: get_field(52)?.clone(),
        analysis_attribute_11: get_field(53)?.clone(),
        analysis_attribute_12: get_field(54)?.clone(),
        analysis_attribute_13: get_field(55)?.clone(),
        analysis_attribute_14: get_field(56)?.clone(),
        analysis_attribute_15: get_field(57)?.clone(),
    })
}

fn read_service_order_transactions(path: &str) -> Result<Vec<ServiceOrderTransaction>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut transactions = Vec::new();
    let mut headers: Option<Vec<String>> = None;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let fields = parse_csv_line(&line);

        if headers.is_none() {
            headers = Some(fields);
            continue;
        }

        if let Some(headers) = &headers {
            if let Some(so_trans) = parse_service_order_transaction(&fields, headers) {
                transactions.push(so_trans);
            }
        }
    }

    Ok(transactions)
}

fn parse_service_order_transaction(fields: &[String], headers: &[String]) -> Option<ServiceOrderTransaction> {
    let mut get_field = |index: usize| -> Option<&String> {
        if index < headers.len() {
            Some(&fields[index])
        } else {
            None
        }
    };

    let get_float = |index: usize| -> Option<f64> {
        get_field(index).and_then(|s| parse_float(s))
    };

    let get_int = |index: usize| -> Option<i32> {
        get_field(index).and_then(|s| parse_int(s))
    };

    Some(ServiceOrderTransaction {
        sro: get_field(0)?.clone(),
        trans_date: get_field(1)?.clone(),
        post_date: get_field(2)?.clone(),
        posted: get_int(3)?,
        bill_status: get_field(4)?.clone(),
        invoice: get_field(5)?.clone(),
        partner: get_field(6)?.clone(),
        work_code: get_field(7)?.clone(),
        ext_cost: get_float(8)?,
        hours_worked: get_float(9)?,
        hours_to_bill: get_float(10)?,
        cost: get_float(11)?,
        ext_price: get_float(12)?,
        work_code_description: get_field(13)?.clone(),
        sro_description: get_field(14)?.clone(),
        billing_code: get_field(15)?.clone(),
        variable_overhead_cost: get_float(16)?,
        labor_rate: get_float(17)?,
        reimbursement_method: get_field(18)?.clone(),
        fixed_overhead_cost: get_float(19)?,
        start_time: get_field(20)?.clone(),
        end_time: get_field(21)?.clone(),
        line_description: get_field(22)?.clone(),
        oper_description: get_field(23)?.clone(),
        trans_type: get_field(24)?.clone(),
        trans_num: get_field(25)?.clone(),
        line_number: get_int(26)?,
        oper_number: get_int(27)?,
        description: get_field(28)?.clone(),
        reimbursement_tax_code_1: get_field(29)?.clone(),
        payroll: get_field(30)?.clone(),
        reimburse_labor: get_float(31)?,
        discount: get_float(32)?,
        reimbursement_tax: get_float(33)?,
        sign_off_date: get_field(34)?.clone(),
        tax_code: get_field(35)?.clone(),
        sign_off: get_field(36)?.clone(),
        cost_currency: get_field(37)?.clone(),
        price_currency: get_field(38)?.clone(),
        reimbursement_currency: get_field(39)?.clone(),
        dept: get_field(40)?.clone(),
        warehouse: get_field(41)?.clone(),
        amount_format_1: get_field(42)?.clone(),
        amount_format_2: get_field(43)?.clone(),
        bill_hold: get_int(44)?,
    })
}

fn read_material_transactions(path: &str) -> Result<Vec<MaterialTransaction>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut transactions = Vec::new();
    let mut headers: Option<Vec<String>> = None;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let fields = parse_csv_line(&line);

        if headers.is_none() {
            headers = Some(fields);
            continue;
        }

        if let Some(headers) = &headers {
            if let Some(mat_trans) = parse_material_transaction(&fields, headers) {
                transactions.push(mat_trans);
            }
        }
    }

    Ok(transactions)
}

fn parse_material_transaction(fields: &[String], headers: &[String]) -> Option<MaterialTransaction> {
    let mut get_field = |index: usize| -> Option<&String> {
        if index < headers.len() {
            Some(&fields[index])
        } else {
            None
        }
    };

    let get_float = |index: usize| -> Option<f64> {
        get_field(index).and_then(|s| parse_float(s))
    };

    let get_int = |index: usize| -> Option<i32> {
        get_field(index).and_then(|s| parse_int(s))
    };

    Some(MaterialTransaction {
        sro: get_field(0)?.clone(),
        sro_description: get_field(1)?.clone(),
        item: get_field(2)?.clone(),
        item_description: get_field(3)?.clone(),
        unit: get_field(4)?.clone(),
        customer: get_field(5)?.clone(),
        exchange_date: get_field(6)?.clone(),
        trans_date: get_field(7)?.clone(),
        post_date: get_field(8)?.clone(),
        billing_code: get_field(9)?.clone(),
        packing_slip: get_field(10)?.clone(),
        posted: get_int(11)?,
        bill_status: get_field(12)?.clone(),
        invoice: get_field(13)?.clone(),
        quantity: get_float(14)?,
        ext_cost: get_float(15)?,
        oper_description: get_field(16)?.clone(),
        line_description: get_field(17)?.clone(),
        matl_cost: get_float(18)?,
        labor_cost: get_float(19)?,
        vovhd_cost: get_float(20)?,
        unit_cost: get_float(21)?,
        location: get_field(22)?.clone(),
        line_number: get_int(23)?,
        trans_num: get_field(24)?.clone(),
        oper_number: get_int(25)?,
        discount: get_float(26)?,
        impact_inventory: get_int(27)?,
        item_exists: get_int(28)?,
        revision: get_field(29)?.clone(),
        unit_price: get_float(30)?,
        ext_price: get_float(31)?,
        um: get_field(32)?.clone(),
        dept: get_field(33)?.clone(),
        partner: get_field(34)?.clone(),
        drop_ship_to: get_field(35)?.clone(),
        cost_type: get_field(36)?.clone(),
        trans_type: get_field(37)?.clone(),
        tax_code: get_field(38)?.clone(),
        outside_cost: get_float(39)?,
        qty_back_ordered: get_float(40)?,
        reimbursement_tax: get_float(41)?,
        fovhd_cost: get_float(42)?,
        reason_code: get_field(43)?.clone(),
        document_number: get_field(44)?.clone(),
        transaction_restriction_code: get_field(45)?.clone(),
        customer_item: get_field(46)?.clone(),
        lot: get_field(47)?.clone(),
        reimbursement_tax_code_1: get_field(48)?.clone(),
        reimbursement_currency: get_field(49)?.clone(),
        price_code: get_field(50)?.clone(),
        sign_off: get_field(51)?.clone(),
        sign_off_date: get_field(52)?.clone(),
        notc: get_field(53)?.clone(),
        notc_description: get_field(54)?.clone(),
        delivery_terms: get_field(55)?.clone(),
        delivery_terms_description: get_field(56)?.clone(),
        origin: get_field(57)?.clone(),
        eu_code: get_field(58)?.clone(),
        commodity: get_field(59)?.clone(),
        process_indicator: get_field(60)?.clone(),
        supply_units: get_float(61)?,
        usage_type: get_field(62)?.clone(),
        address: get_field(63)?.clone(),
        reimbursement_method: get_field(64)?.clone(),
        s_n_prefix: get_field(65)?.clone(),
        lot_prefix: get_field(66)?.clone(),
        reimburse_material: get_float(67)?,
        lot_tracked: get_int(68)?,
        serial_tracked: get_int(69)?,
        bill_hold: get_int(70)?,
    })
}

fn read_misc_transactions(path: &str) -> Result<Vec<MiscTransaction>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut transactions = Vec::new();
    let mut headers: Option<Vec<String>> = None;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let fields = parse_csv_line(&line);

        if headers.is_none() {
            headers = Some(fields);
            continue;
        }

        if let Some(headers) = &headers {
            if let Some(misc_trans) = parse_misc_transaction(&fields, headers) {
                transactions.push(misc_trans);
            }
        }
    }

    Ok(transactions)
}

fn parse_misc_transaction(fields: &[String], headers: &[String]) -> Option<MiscTransaction> {
    let mut get_field = |index: usize| -> Option<&String> {
        if index < headers.len() {
            Some(&fields[index])
        } else {
            None
        }
    };

    let get_float = |index: usize| -> Option<f64> {
        get_field(index).and_then(|s| parse_float(s))
    };

    let get_int = |index: usize| -> Option<i32> {
        get_field(index).and_then(|s| parse_int(s))
    };

    Some(MiscTransaction {
        sro: get_field(0)?.clone(),
        trans_date: get_field(1)?.clone(),
        post_date: get_field(2)?.clone(),
        posted: get_int(3)?,
        bill_status: get_field(4)?.clone(),
        invoice: get_field(5)?.clone(),
        billing_code: get_field(6)?.clone(),
        misc_code: get_field(7)?.clone(),
        misc_code_description: get_field(8)?.clone(),
        quantity: get_float(9)?,
        ext_cost: get_float(10)?,
        ext_price: get_float(11)?,
        matl_cost: get_float(12)?,
        reimbursement_total_due: get_float(13)?,
        oper_description: get_field(14)?.clone(),
        sro_description: get_field(15)?.clone(),
        line_description: get_field(16)?.clone(),
        invoice_1: get_field(17)?.clone(),
        unit_cost: get_float(18)?,
        partner: get_field(19)?.clone(),
        payment_type: get_field(20)?.clone(),
        labor_cost: get_float(21)?,
        vovhd_cost: get_float(22)?,
        discount: get_float(23)?,
        unit_price: get_float(24)?,
        misc_pay_type_reimbursable_grid_col: get_int(25)?,
        tax_code: get_field(26)?.clone(),
        reimbursement_tax_code_1: get_field(27)?.clone(),
        bill_hold: get_int(28)?,
        cost_currency: get_field(29)?.clone(),
        reimbursement_tax: get_float(30)?,
        outside_cost: get_float(31)?,
        fovhd_cost: get_float(32)?,
        line_number: get_int(33)?,
        oper_number: get_int(34)?,
        trans_num: get_field(35)?.clone(),
        trans_type: get_field(36)?.clone(),
        dept: get_field(37)?.clone(),
        sign_off: get_field(38)?.clone(),
        sign_off_date: get_field(39)?.clone(),
        warehouse: get_field(40)?.clone(),
        reimbursement_status: get_field(41)?.clone(),
        reimbursement_currency: get_field(42)?.clone(),
        price_currency: get_field(43)?.clone(),
        amount_format_1: get_field(44)?.clone(),
        amount_format_2: get_field(45)?.clone(),
        amount_format_3: get_field(46)?.clone(),
    })
}

fn create_tie_out_report(
    gl_transactions: &[GLPostedTransaction],
    labor_transactions: &[ServiceOrderTransaction],
    material_transactions: &[MaterialTransaction],
    misc_transactions: &[MiscTransaction],
) -> String {
    let mut report = String::new();

    // Header
    report.push_str("GL Tie-Out Report\n");
    report.push_str("================\n\n");

    // Summary
    report.push_str("Summary:\n");
    report.push_str(&format!("GL Transactions: {}\n", gl_transactions.len()));
    report.push_str(&format!("Labor Transactions: {}\n", labor_transactions.len()));
    report.push_str(&format!("Material Transactions: {}\n", material_transactions.len()));
    report.push_str(&format!("Misc Transactions: {}\n\n", misc_transactions.len()));

    // Detailed Tie-Out
    report.push_str("Detailed Tie-Out:\n");
    report.push_str("----------------\n\n");

    // Group GL transactions by invoice
    let mut gl_by_invoice: HashMap<String, Vec<&GLPostedTransaction>> = HashMap::new();
    for gl in gl_transactions {
        if let Some(invoice) = &gl.invoice_voucher {
            gl_by_invoice.entry(invoice.clone()).or_default().push(gl);
        }
    }

    // Group service orders by SRO
    let mut labor_by_sro: HashMap<String, Vec<&ServiceOrderTransaction>> = HashMap::new();
    for labor in labor_transactions {
        labor_by_sro.entry(labor.sro.clone()).or_default().push(labor);
    }

    let mut material_by_sro: HashMap<String, Vec<&MaterialTransaction>> = HashMap::new();
    for material in material_transactions {
        material_by_sro.entry(material.sro.clone()).or_default().push(material);
    }

    let mut misc_by_sro: HashMap<String, Vec<&MiscTransaction>> = HashMap::new();
    for misc in misc_transactions {
        misc_by_sro.entry(misc.sro.clone()).or_default().push(misc);
    }

    // Process each SRO
    for (sro, labor_list) in &labor_by_sro {
        report.push_str(&format!("SRO: {}\n", sro));
        report.push_str("--------------------------------------------------\n");

        // Labor transactions for this SRO
        if let Some(labor_list) = labor_by_sro.get(sro) {
            report.push_str("\nLabor Transactions:\n");
            for labor in labor_list {
                report.push_str(&format!("  Work Code: {}, Cost: {:.2}, Ext Cost: {:.2}\n",
                    labor.work_code, labor.cost, labor.ext_cost));
            }
        }

        // Material transactions for this SRO
        if let Some(material_list) = material_by_sro.get(sro) {
            report.push_str("\nMaterial Transactions:\n");
            for material in material_list {
                report.push_str(&format!("  Item: {}, Cost: {:.2}, Ext Cost: {:.2}\n",
                    material.item, material.unit_cost, material.ext_cost));
            }
        }

        // Misc transactions for this SRO
        if let Some(misc_list) = misc_by_sro.get(sro) {
            report.push_str("\nMisc Transactions:\n");
            for misc in misc_list {
                report.push_str(&format!("  Misc Code: {}, Cost: {:.2}, Ext Cost: {:.2}\n",
                    misc.misc_code, misc.unit_cost, misc.ext_cost));
            }
        }

        // GL transactions for this SRO
        report.push_str("\nGL Transactions:\n");
        for gl in gl_transactions {
            if let Some(invoice) = &gl.invoice_voucher {
                if invoice == sro {
                    report.push_str(&format!("  Account: {}, Debit: {:.2}, Credit: {:.2}\n",
                        gl.account, gl.debit_domestic, gl.credit_domestic));
                }
            }
        }

        report.push_str("\n");
    }

    report
}

fn main() -> Result<(), Box<dyn Error>> {
    let data_path = Path::new("data");

    // Read GL transactions
    let gl_path = data_path.join("GLPostedTransactions_51120.csv");
    let gl_transactions = read_gl_transactions(&gl_path)?;

    // Read service order transactions
    let labor_path = data_path.join("LABOR_ServiceOrderTransactions.csv");
    let labor_transactions = read_service_order_transactions(&labor_path)?;

    let material_path = data_path.join("MATERIAL_ServiceOrderTransactions.csv");
    let material_transactions = read_material_transactions(&material_path)?;

    let misc_path = data_path.join("MISC_ServiceOrderTransactions.csv");
    let misc_transactions = read_misc_transactions(&misc_path)?;

    // Create tie-out report
    let report = create_tie_out_report(&gl_transactions, &labor_transactions, &material_transactions, &misc_transactions);

    // Write report to file
    let output_path = Path::new("output");
    std::fs::create_dir_all(output_path)?;

    let report_path = output_path.join("gl_tie_out_report.txt");
    let mut report_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&report_path)?;

    writeln!(report_file, "{}", report)?;

    println!("GL tie-out report generated successfully!");
    println!("Report saved to: {:?}", report_path);

    Ok(())
}
