# sev-attest-tool

This library generates and verifies SEV-SNP attestation reports.

## Python

To build the Python package, run:

```
sudo docker run --env MATURIN_PYPI_TOKEN=$MATURIN_PYPI_TOKEN --rm -v $(pwd):/io --entrypoint "" -it ghcr.io/pyo3/maturin bash -c 'yum install -y openssl-devel && maturin publish --compatibility manylinux2014'
```