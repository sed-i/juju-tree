# juju-tree
Juju plugin for printing controllers and models in tree view.

## Install
```
cargo install --git https://github.com/sed-i/juju-tree
```

## Usage
```
$ juju-tree
```

Sample output:
```
  lxd (3.2.0, lxd)
      admin/controller
    * admin/welcome-k8s
      admin/welcome-lxd
* microk8s (3.2.0, kubernetes)
      admin/controller
      admin/test-external-url-s4lh
      admin/test-get-password-bwuv
      admin/test-kubectl-delete-jizp
      admin/test-rescale-charm-lxwr
```
